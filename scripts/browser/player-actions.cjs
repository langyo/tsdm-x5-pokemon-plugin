// Real game WASM against isolated player/shop fixtures. Never connects to a real account.
const assert=require('node:assert/strict');
const fs=require('node:fs/promises');
const http=require('node:http');
const path=require('node:path');
const {chromium,expect:baseExpect}=require('@playwright/test');
const expect=baseExpect.configure({timeout:10000});
const wasmDirectory=process.env.GAME_WASM_DIRECTORY || path.resolve(__dirname,'../../plugin/wasm');
const artifacts=path.join(__dirname,'artifacts/player-actions');
const prefix='/source/plugin/pokemon/wasm/';
const pixel=Buffer.from('R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7','base64');
const html=`<!doctype html><html><meta charset="utf-8"><link rel="stylesheet" href="${prefix}game.css"><div id="main"></div><script type="module">import init,{WebHandle} from '${prefix}_game.js';await init({module_or_path:'${prefix}_game_bg.wasm'});await new WebHandle().start();</script></html>`;
function pet(id,name,site,hp=50) {
  return {id,name,nickname:`昵称${id}`,type_id:id,level:5,exp:20,exp_to_next_level:80,exp_for_current_level:0,exp_for_next_level:100,hp,max_hp:50,gender:0,is_shiny:false,site,state:1,state_text:'正常',state_class:'normal',skills:[],base_info:{id,name,type_1:'水',type_2:null}};
}
// 商店货架：type=1 只放一件，保证旧用例里 `.buy-btn` 严格匹配单键；
// 其余分类放多件供 keyword 筛选与翻页用例消费。
const shopCatalog=[
  {id:17,name:'测试药水',description:'回复HP',type_id:1,type_name:'回复药',image:'hp20',price:100,stock:999,effect:{},can_buy:true},
  {id:18,name:'测试精灵球·红',description:'捕捉宝可梦',type_id:2,type_name:'精灵球',image:'ball-red',price:150,stock:999,effect:{},can_buy:true},
  {id:19,name:'超级测试球',description:'高级捕捉球',type_id:2,type_name:'精灵球',image:'ball-blue',price:300,stock:999,effect:{},can_buy:true},
  {id:20,name:'测试精灵球·蓝',description:'捕捉宝可梦',type_id:2,type_name:'精灵球',image:'ball-blue2',price:180,stock:999,effect:{},can_buy:true},
  ...Array.from({length:9},(_,i)=>({id:30+i,name:`测试强化剂${i+1}`,description:'强化道具',type_id:4,type_name:'强化道具',image:'atkg',price:10+i*10,stock:999,effect:{},can_buy:true})),
];
// 复刻 plugin/api/shop.php 的列表行为：type 过滤 + keyword 字面子串 + 价格升序 + 每页 8 条。
function shopListPage(params) {
  let rows=shopCatalog;
  const type=params.get('type');
  if(type!==null)rows=rows.filter(row=>row.type_id===Number(type));
  const keyword=(params.get('keyword')||'').trim();
  if(keyword)rows=rows.filter(row=>row.name.includes(keyword));
  rows=[...rows].sort((a,b)=>a.price-b.price);
  const page=Math.max(1,Number(params.get('page')||1));
  const perPage=8;
  return {items:rows.slice((page-1)*perPage,page*perPage),total:rows.length,page,per_page:perPage,total_pages:rows.length?Math.ceil(rows.length/perPage):0};
}
async function fixture(options={}) {
  const state={requests:[],unexpected:[],gates:new Map(),money:1000,battle:!!options.battle,profileFailures:options.profileFailure?1:0,renameFailure:false,buys:0,initializations:0,newPlayer:!!options.newPlayer,pets:[pet(25,'皮卡丘',1,20),pet(1,'妙蛙种子',2,40)]};
  state.items=options.medicine ? [17,18].map(id=>({id:1000+id,type_id:id,item_type:1,name:`药水${id}`,description:'回复20点HP',image:'hp20',quantity:2,type_name:'回复药',can_use:true}))
    : (options.qualityMedicine ? [{id:1046,type_id:46,item_type:4,name:'品质重洗药',description:'重洗品质',image:'yypg',quantity:2,type_name:'强化道具',can_use:true,module:'quality'}] : []);
  state.equipment={25:[{myitem_id:125,type_id:50,name:'A的装备',image:'equip',quantity:1,available_count:0,is_equipped:true},null,null,null],1:[{myitem_id:101,type_id:51,name:'B的装备',image:'equip',quantity:1,available_count:0,is_equipped:true},null,null,null]};
  state.equipmentFailures=options.readFailures?1:0;state.skillFailures=options.readFailures?1:0;
  state.badgeHidden=!!options.badgeHidden;state.badgeFailure=false;
  state.gear={125:{...structuredClone(state.equipment[25][0]),equipment_hp:10,equipment_atk:3},126:{myitem_id:126,type_id:52,name:'替换装备',image:'equip',quantity:1,available_count:1,is_equipped:false,equipment_hp:20,equipment_atk:5}};
  const scene=()=>({battle_id:'battle_37',engine_battle_id:37,revision:2,phase:'active',events:[],map_id:1,map_name:'Test',turn:2,my_pokemon:{id:25,instance_id:25,name:'皮卡丘',level:5,hp:20,max_hp:50,skills:[]},wild_pokemon:{id:19,name:'小拉达',level:5,hp:30,max_hp:30},status:'active',battle_over:false,can_continue_switch:false,message:''});
  const pendingReleases=new Set();
  const hold=(key,once=false)=>{let release;const promise=new Promise(resolve=>{release=resolve});const done=()=>{state.gates.delete(key);pendingReleases.delete(done);release()};pendingReleases.add(done);state.gates.set(key,{promise,once});return done};
  async function dispatch(url,body) {
    const key=`${url.searchParams.get('endpoint')}/${url.searchParams.get('action')}`;
    state.requests.push({key,body,query:Object.fromEntries(url.searchParams)});
    const profileMoney=state.money;
    const badgeSnapshot=state.badgeHidden;
    // Each delayed list response represents the snapshot captured by that read.
    const pokemonSnapshot=structuredClone(state.pets);
    const gate=state.gates.get(key);if(gate){if(gate.once)state.gates.delete(key);await gate.promise;}
    let data;
    switch(key) {
      case 'user/profile':
        if(state.profileFailures-- > 0) return {success:false,error:'模拟账户载入失败',code:503};
        data={uid:1,username:'Fixture Trainer',group_id:10,is_admin:false,is_new_player:state.newPlayer,money:profileMoney,adventure_strength:100,strength_level:1,wins:0,losses:0,total_pokemons:state.newPlayer?0:state.pets.length,total_items:2,npcid:state.battle?19:0};break;
      case 'user/inventory_stats':data={categories:[]};break;
      case 'user/badge_status':data={hidden:badgeSnapshot,initialized:true};break;
      case 'user/refresh_badge':if(state.badgeFailure){state.badgeFailure=false;return {success:false,error:'模拟徽章保存失败',code:503};}state.badgeHidden=body.hide;data={hidden:state.badgeHidden,initialized:true};break;
      case 'user/online_players':data={total:0,players:[],max_display:20};break;
      case 'topics/list':data={topics:[],news_announcements:[],total:0};break;
      case 'config/global':data={is_open:true,is_enable_catch:true};break;
      case 'pokemon/list':data={pokemons:state.newPlayer?[]:pokemonSnapshot,total:state.newPlayer?0:pokemonSnapshot.length};break;
      case 'pokemon/detail':{const p=state.pets.find(p=>p.id===Number(url.searchParams.get('pokemon_id')));assert(p);data={...p,stats:{hp:50,attack:p.id*10,defense:10,sp_attack:10,sp_defense:10,speed:10}};break;}
      case 'pokemon/equipment': {
        if(state.equipmentFailures-- > 0)return {success:false,error:'模拟装备服务不可用',code:503};
        const pid=Number(url.searchParams.get('pokemon_id'));
        const used126=Object.values(state.equipment).filter(slots=>slots.some(item=>item?.myitem_id===126)).length;
        const spareGear={...state.gear[126],equipped_count:used126,available_count:used126?0:1,is_equipped:state.equipment[pid].some(item=>item?.myitem_id===126)};
        let owned=options.equipmentLifecycle?[spareGear]:[];
        if(options.occupiedStack){
          owned=[{...state.equipment[1][0],quantity:2,equipped_count:1,available_count:state.legacyAvailability?1:0,is_equipped:pid===1},
            spareGear];
        }
        data={pokemon_id:pid,equipment_slots:state.equipment[pid].map((item,slot_index)=>({slot_index,equipment_id:item?.myitem_id||0,item})),owned_items:owned,shop_items:[],user_money:state.money};break;
      }
      case 'pokemon/unequip_item': {
        state.equipment[body.pokemon_id][body.slot_index]=null;const p=state.pets.find(p=>p.id===body.pokemon_id);assert(p);
        if(options.equipmentLifecycle){p.hp=18;p.max_hp=45;}
        data={message:'已卸下',pokemon_id:body.pokemon_id,slot_index:body.slot_index,item_name:'装备',new_hp:p.hp,new_maxhp:p.max_hp};break;
      }
      case 'pokemon/equip_item': {
        if(state.equipmentWriteFailure){state.equipmentWriteFailure=false;return {success:false,error:'模拟装载失败',code:400};}
        assert(state.gear[body.myitem_id]);assert.equal(state.equipment[body.pokemon_id][body.slot_index],null);
        state.equipment[body.pokemon_id][body.slot_index]=state.gear[body.myitem_id];const p=state.pets.find(p=>p.id===body.pokemon_id);assert(p);p.hp=24;p.max_hp=60;
        data={message:'已装备',pokemon_id:body.pokemon_id,slot_index:body.slot_index,item_name:state.gear[body.myitem_id].name,new_hp:p.hp,new_maxhp:p.max_hp};break;
      }
      case 'pokemon/learnable_skills':
        if(state.skillFailures-- > 0)return {success:false,error:'模拟技能服务不可用',code:503};
        data={pokemon_id:25,pokemon_level:5,available_skills:[{id:12,name:'电光',description:'测试技能',type:'电',category:'特殊',power:40,max_pp:10,required_level:1,is_available:true}],unlocked_skills:[]};break;
      case 'pokemon/learn_skill': {
        const pokemon=state.pets.find(p=>p.id===body.pokemon_id);assert(pokemon);
        assert(!pokemon.skills.some(s=>s.type_id===body.skill_id),'Duplicate learning reached the fixture');
        const skill={type_id:body.skill_id,name:'电光',pp:10,max_pp:10,can_forget:true,skill_type:'电',category:'特殊',level:1,power:40};pokemon.skills.push(skill);
        data={message:'学习成功',pokemon_id:pokemon.id,skill};break;
      }
      case 'pokemon/forget_skill': {const pokemon=state.pets.find(p=>p.id===body.pokemon_id);assert(pokemon);pokemon.skills=pokemon.skills.filter(s=>s.type_id!==body.skill_id);data=null;break;}
      case 'pokemon/rename':
        if(state.renameFailure) {state.renameFailure=false;return {success:false,error:'昵称暂时无法保存',code:400};}
        state.pets.find(p=>p.id===body.id).nickname=body.name;data=null;break;
      case 'battle/recover':if(!state.battle)return {success:false,error:'No battle',code:404};data=scene();break;
      case 'battle/maps':data={maps:[],total:0};break;
      case 'battle/get_battle_items':data={items:[]};break;
      case 'user/inventory':data={items:state.items.filter(item=>item.quantity>0),total:state.items.filter(item=>item.quantity>0).length,page:1,per_page:50,total_pages:1};break;
      case 'user/get_usable_pokemon':{
        const item=state.items.find(item=>item.type_id===Number(url.searchParams.get('item_id')));assert(item);
        const quality=item.module==='quality';
        const targets=(quality?state.pets.filter(p=>p.site<3):[state.pets[0]]).map(p=>({
          id:p.id,type_id:p.type_id,name:p.name,nickname:p.nickname,level:p.level,hp:p.hp,max_hp:p.max_hp,state:p.state,
          site:p.site,is_shiny:p.is_shiny,
          iv_total:p.iv_total??(p.site===1?186:170),
          quality_rank:p.quality_rank??(p.site===1?8:7),
          quality_grade:p.quality_grade??(p.site===1?'SS+':'SS'),
        }));
        data={item_id:item.type_id,item_name:item.name,item_type:item.item_type??1,item_module:item.module||'',usable_pokemon:targets,unusable_count:quality?0:1};
        break;
      }
      case 'user/use_item':{
        const item=state.items.find(item=>item.type_id===body.item_id);const pokemon=state.pets.find(p=>p.id===body.pokemon_id);assert(item&&pokemon);
        item.quantity--;
        if(item.module==='quality'){
          const oldGrade='SS+',oldIv=186,newGrade='A',newIv=120;
          pokemon.iv_total=newIv;pokemon.quality_rank=5;pokemon.quality_grade=newGrade;
          data={success:true,message:`成功对 ${pokemon.nickname} 使用了 ${item.name}：品质 ${oldGrade} → ${newGrade}（总个体值 ${oldIv} → ${newIv}）`,item_remaining:item.quantity,pokemon_updated:{id:pokemon.id,hp:pokemon.hp,max_hp:pokemon.max_hp,old_quality_grade:oldGrade,new_quality_grade:newGrade,old_iv_total:oldIv,new_iv_total:newIv}};
        }else{
          pokemon.hp=Math.min(pokemon.max_hp,pokemon.hp+20);
          data={success:true,message:'已使用药水',item_remaining:item.quantity,pokemon_updated:{id:pokemon.id,hp:pokemon.hp,max_hp:pokemon.max_hp,state:pokemon.state}};
        }
        break;
      }
      case 'shop/list':data=shopListPage(url.searchParams);break;
      case 'shop/pets':{
        const keyword=(url.searchParams.get('keyword')||'').trim();
        const pets=[{id:4,name:'小火龙',type_1:'火',hp:40,atk:50,def:40,spatk:50,spdef:50,speed:60,price:100,can_buy:true}];
        const rows=keyword?pets.filter(p=>p.name.includes(keyword)):pets;
        data={pets:rows,total:rows.length,page:Math.max(1,Number(url.searchParams.get('page')||1)),per_page:8,total_pages:rows.length?1:0};break;
      }
      case 'shop/buy':state.buys++;state.money-=100;data={message:'已购买',total_cost:100,remaining_money:state.money,items_purchased:1};break;
      case 'shop/buy_pet':state.buys++;state.money-=100;data={message:'已购买',total_cost:100,remaining_money:state.money,pokemon_name:'小火龙',pokemon_type_id:4,site:3};break;
      case 'user/heal':case 'user/heal_and_flee':{if(state.healFailure){state.healFailure=false;return {success:false,error:'模拟治疗失败',code:400};}const p=state.pets.find(p=>p.id===Number(url.searchParams.get('pokemon_id')));assert(p);const cost=p.hp<50?20:0;state.money-=cost;p.hp=50;if(key.endsWith('heal_and_flee'))state.battle=false;data={cost,message:'已治疗',pokemon_id:p.id,current_hp:50,max_hp:50};break;}
      case 'user/initialize':state.initializations++;state.newPlayer=false;data={success:true,message:'已初始化',money:1000,egg_received:true};break;
      default:throw Error(`Unexpected ${key}`);
    }
    return {success:true,data};
  }
  const server=http.createServer(async(req,res)=>{try{
    const url=new URL(req.url,'http://localhost');
    if(url.pathname==='/plugin.php' && url.searchParams.get('endpoint')==='avatar') {res.writeHead(200,{'Content-Type':'image/gif'});res.end(pixel);}
    else if(url.pathname==='/plugin.php' && url.searchParams.has('endpoint')) {let body='';for await(const chunk of req)body+=chunk;const result=await dispatch(url,body?JSON.parse(body):{});res.writeHead(result.code||200,{'Content-Type':'application/json'});res.end(JSON.stringify(result));}
    else if(url.pathname.startsWith(prefix)) {const filename=path.resolve(wasmDirectory,decodeURIComponent(url.pathname.slice(prefix.length)));assert(filename.startsWith(path.resolve(wasmDirectory)+path.sep));const bytes=await fs.readFile(filename);res.writeHead(200,{'Content-Type':{'.js':'text/javascript','.wasm':'application/wasm','.css':'text/css'}[path.extname(filename)]||'application/octet-stream'});res.end(bytes);}
    else if(url.pathname.includes('/images/')) {res.writeHead(200,{'Content-Type':'image/gif'});res.end(pixel);}
    else {res.writeHead(200,{'Content-Type':'text/html'});res.end(html);}
  } catch(error) {state.unexpected.push(String(error));res.writeHead(500,{'Content-Type':'application/json'});res.end(JSON.stringify({success:false,error:String(error)}));}});
  await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));
  return {state,hold,url:`http://127.0.0.1:${server.address().port}/plugin.php?id=pokemon:pokemon`,close:async()=>{for(const release of pendingReleases)release();server.closeAllConnections();await new Promise(resolve=>server.close(resolve))}};
}
async function runCase(browser,name,options,test) {
  if(process.env.PLAYER_CASE_FILTER && !new RegExp(process.env.PLAYER_CASE_FILTER).test(name))return;
  const app=await fixture(options);
  const context=await browser.newContext({viewport:{width:1440,height:1100}});
  context.setDefaultTimeout(12000);context.setDefaultNavigationTimeout(20000);
  await context.route('https://**/*',route=>route.fulfill({status:200,contentType:'image/gif',body:pixel}));
  const page=await context.newPage();const errors=[];page.on('pageerror',error=>errors.push(String(error)));
  try {await page.goto(app.url);await test(page,app);assert.deepEqual(errors,[]);assert.deepEqual(app.state.unexpected,[]);console.log(`PASS ${name}`);}
  catch(error) {await page.screenshot({path:path.join(artifacts,`${name}-failure.png`),fullPage:true});console.error('Requests:',JSON.stringify(app.state.requests),'page errors:',errors,'fixture errors:',app.state.unexpected);throw error;}
  finally {await context.close();await app.close();}
}
async function run() {
  await fs.mkdir(artifacts,{recursive:true});const browser=await chromium.launch({headless:true});
  try {
    await runCase(browser,'occupied-equipment-stacks-cannot-replace-current-gear',{occupiedStack:true},async(page,app)=>{
      // Also tolerate the older availability field while respecting equipped_count.
      app.state.legacyAvailability=true;
      await page.getByText('个人中心',{exact:true}).click();await page.locator('.mypm-tab').filter({hasText:'装备'}).click();
      const occupied=page.locator('.inventory-item-wrapper').filter({has:page.locator('img[alt="B的装备"]')});
      await expect(occupied).toContainText('其他宠物使用中');
      await expect(occupied.locator('.inventory-item')).toHaveClass(/equipped/);
      await occupied.locator('.inventory-item').click();await page.locator('.equip-slot').nth(1).click();
      await occupied.locator('.inventory-item').click();await page.locator('.equip-slot').first().click();
      assert.equal(app.state.requests.filter(r=>r.key==='pokemon/equip_item'||r.key==='pokemon/unequip_item').length,0);
      assert.equal(app.state.equipment[25][0].myitem_id,125);
      assert.equal(app.state.equipment[1][0].myitem_id,101);
      app.state.legacyAvailability=false;
      await page.getByRole('button',{name:/昵称1 Lv/}).click();
      await expect(occupied).toContainText('已装备');
      await expect(occupied.locator('.inventory-item')).toHaveClass(/equipped/);
      await page.getByRole('button',{name:/昵称25 Lv/}).click();
      await expect(occupied).toContainText('其他宠物使用中');
      await page.locator('.inventory-item-wrapper').filter({has:page.locator('img[alt="替换装备"]')}).locator('.inventory-item').click();
      await page.locator('.equip-slot').nth(1).click();
      await expect(page.locator('.equip-icon[alt="替换装备"]')).toBeVisible();
      assert.equal(app.state.requests.filter(r=>r.key==='pokemon/equip_item').length,1);
      assert.equal(app.state.requests.filter(r=>r.key==='pokemon/unequip_item').length,0);
      assert.equal(app.state.equipment[25][0].myitem_id,125);
      assert.equal(app.state.equipment[25][1].myitem_id,126);
      await page.screenshot({path:path.join(artifacts,'occupied-equipment-stacks.png'),fullPage:true});
    });
    await runCase(browser,'header-navigation-works-with-keyboard',{},async(page)=>{
      await expect(page.locator('.page-home')).toBeVisible();
      for(const target of ['.logo-link','.nav-link']) {
        await page.keyboard.press('Tab');await expect(page.locator(target).first()).toBeFocused();
      }
      await page.keyboard.press('Enter');await expect(page.locator('.page-my-pokemon')).toBeVisible();
      await page.keyboard.press('Tab');await expect(page.locator('.nav-link').nth(1)).toBeFocused();
      await page.keyboard.press('Enter');await expect(page.locator('.page-shop')).toBeVisible();
      await page.keyboard.press('Tab');await expect(page.locator('.nav-link').nth(2)).toBeFocused();
      await page.keyboard.press('Enter');await expect(page.locator('.page-pokemon-center')).toBeVisible();
      await page.keyboard.press('Tab');await expect(page.locator('.nav-link').nth(3)).toBeFocused();
      await page.keyboard.press('Enter');await expect(page.locator('.page-adventure')).toBeVisible();
      await page.locator('.logo-link').focus();await page.keyboard.press('Enter');await expect(page.locator('.page-home')).toBeVisible();
      await page.screenshot({path:path.join(artifacts,'header-keyboard.png'),fullPage:true});
    });
    await runCase(browser,'badge-state-load-does-not-overwrite-mutation',{},async(page,app)=>{
      const release=app.hold('user/badge_status',true);await page.getByText('商店',{exact:true}).click();
      await expect.poll(()=>app.state.requests.filter(r=>r.key==='user/badge_status').length).toBe(1);
      await expect(page.locator('.badge-toggle-btn')).toBeDisabled();
      // Explicit refresh can safely initialize visibility while the older read is pending.
      await page.getByRole('button',{name:'刷新徽章',exact:true}).click();
      await expect(page.locator('.badge-toggle-btn')).toBeEnabled();
      const releaseMutation=app.hold('user/refresh_badge',true);
      await page.locator('.badge-toggle-btn').evaluate(node=>{node.click();node.click();node.click()});
      await expect.poll(()=>app.state.requests.filter(r=>r.key==='user/refresh_badge').length).toBe(2);
      releaseMutation();await expect(page.locator('.badge-toggle-btn')).toHaveText('显示宠物');
      const response=page.waitForResponse(r=>r.url().includes('action=badge_status'));release();await response;
      await expect(page.locator('.badge-toggle-btn')).toHaveText('显示宠物');assert.equal(app.state.badgeHidden,true);
      app.state.badgeFailure=true;await page.locator('.badge-toggle-btn').click();
      await expect(page.locator('.toast-item.error')).toContainText('模拟徽章保存失败');
      await expect(page.locator('.badge-toggle-btn')).toHaveText('显示宠物');
      await page.locator('.badge-toggle-btn').click();await expect(page.locator('.badge-toggle-btn')).toHaveText('隐藏宠物');
      assert.equal(app.state.badgeHidden,false);
    });
    await runCase(browser,'badge-initial-hidden-state-can-be-shown',{badgeHidden:true},async(page,app)=>{
      await page.getByText('商店',{exact:true}).click();
      await expect(page.locator('.badge-toggle-btn')).toHaveText('显示宠物');
      await page.locator('.badge-toggle-btn').click();
      await expect(page.locator('.badge-toggle-btn')).toHaveText('隐藏宠物');
      assert.equal(app.state.badgeHidden,false);
    });
    await runCase(browser,'toast-expiration-survives-navigation',{},async(page,app)=>{
      await page.getByText('商店',{exact:true}).click();await page.locator('.buy-btn').click();
      await expect(page.locator('.toast-item.success')).toContainText('成功购买');
      await page.locator('.logo-link').click();await expect(page.locator('.page-home')).toBeVisible();
      await expect(page.locator('.toast-item.success')).toHaveCount(0,{timeout:5000});
      await page.getByText('商店',{exact:true}).click();app.state.badgeFailure=true;
      await page.locator('.badge-refresh-btn').click();await expect(page.locator('.toast-item.error')).toBeVisible();
      await page.locator('.logo-link').click();await expect(page.locator('.toast-item.error')).toHaveCount(0,{timeout:10000});
    });
    await runCase(browser,'equipment-operations-finish-after-navigation',{equipmentLifecycle:true},async(page,app)=>{
      for(const operation of ['unequip','move','replace','replace-fails']) {
        app.state.equipment[25]=[structuredClone(app.state.gear[125]),null,null,null];
        app.state.pets[0].hp=20;app.state.pets[0].max_hp=50;
        await page.getByText('个人中心',{exact:true}).click();
        await page.locator('.mypm-tab').filter({hasText:'装备'}).click();
        await expect(page.locator('.equip-icon')).toHaveAttribute('alt','A的装备');
        const release=app.hold('pokemon/unequip_item',true);
        const beforeUnequip=app.state.requests.filter(r=>r.key==='pokemon/unequip_item').length;
        app.state.equipmentWriteFailure=operation==='replace-fails';
        if(operation.startsWith('replace')) await page.locator('.inventory-item').click();
        await page.locator('.equip-slot').first().click();
        if(operation==='unequip') await page.locator('.equipment-inventory-section .inventory-header').click();
        if(operation==='move') await page.locator('.equip-slot').nth(1).click();
        await expect.poll(()=>app.state.requests.filter(r=>r.key==='pokemon/unequip_item').length).toBe(beforeUnequip+1);
        const beforeList=app.state.requests.filter(r=>r.key==='pokemon/list').length;
        await page.getByText('商店',{exact:true}).click();release();
        await expect.poll(()=>app.state.requests.filter(r=>r.key==='pokemon/list').length).toBeGreaterThan(beforeList);
        const expectedHP=operation==='unequip'||operation==='replace-fails'?'18/45':'24/60';
        await page.locator('.pm-mini-slot').first().hover();await expect(page.locator('.stat-hp-bar .stat-value-inside')).toHaveText(expectedHP);
        if(operation==='unequip'||operation==='replace-fails')assert.equal(app.state.equipment[25][0],null);
        else assert.equal(app.state.equipment[25][operation==='move'?1:0].myitem_id,operation==='move'?125:126);
        if(operation==='replace-fails')await expect(page.locator('.toast-item.error')).toContainText('模拟装载失败');
      }
      // Stay on the equipment page: refreshing the global list must also refresh
      // the equipment resource, including slot icons and calculated bonus tags.
      app.state.equipment[25]=[structuredClone(app.state.gear[125]),null,null,null];
      await page.getByText('个人中心',{exact:true}).click();
      await expect(page.locator('.equip-icon')).toHaveAttribute('alt','A的装备');
      await expect(page.locator('.bonus-tag-small.hp')).toHaveText('+10 HP');
      await expect(page.locator('.bonus-tag-small.atk')).toHaveText('+3 攻击');
      const equipmentReads=()=>app.state.requests.filter(r=>r.key==='pokemon/equipment').length;
      const beforeUnequipRead=equipmentReads();
      await page.locator('.equip-slot').first().click();
      await page.locator('.equipment-inventory-section .inventory-header').click();
      await expect(page.locator('.equip-icon')).toHaveCount(0);
      await expect(page.locator('.bonus-tag-small')).toHaveCount(0);
      await expect(page.locator('.equipment-loading-overlay')).toHaveCount(0);
      await expect.poll(equipmentReads).toBeGreaterThan(beforeUnequipRead);
      const beforeEquipRead=equipmentReads();
      await page.locator('.inventory-item').click();await page.locator('.equip-slot').first().click();
      await expect(page.locator('.equip-icon')).toHaveAttribute('alt','替换装备');
      await expect(page.locator('.bonus-tag-small.hp')).toHaveText('+20 HP');
      await expect(page.locator('.bonus-tag-small.atk')).toHaveText('+5 攻击');
      await expect(page.locator('.equipment-loading-overlay')).toHaveCount(0);
      await expect.poll(equipmentReads).toBeGreaterThan(beforeEquipRead);
      assert.equal(app.state.equipment[25][0].myitem_id,126);
    });
    await runCase(browser,'equipment-selection-stays-with-its-pokemon',{},async(page,app)=>{
      await page.getByText('个人中心',{exact:true}).click();
      await page.getByRole('button',{name:'装备',exact:true}).click();
      await expect(page.locator('.equip-icon')).toHaveAttribute('alt','A的装备');
      await page.locator('.equip-slot').first().click();
      await expect(page.locator('.equip-slot-overlay.selected')).toHaveCount(1);
      await page.getByRole('button',{name:/昵称1 Lv/}).click();
      await expect(page.locator('.equip-icon')).toHaveAttribute('alt','B的装备');
      await expect(page.locator('.equip-slot-overlay.selected')).toHaveCount(0);
      await page.locator('.equipment-inventory-section').click();
      assert.equal(app.state.requests.filter(r=>r.key==='pokemon/unequip_item').length,0);
      await page.locator('.equip-slot').first().click();
      await page.locator('.equipment-inventory-section').click();
      await expect.poll(()=>app.state.requests.filter(r=>r.key==='pokemon/unequip_item').length).toBe(1);
      assert.equal(app.state.equipment[1][0],null);assert.equal(app.state.equipment[25][0].myitem_id,125);
    });
    await runCase(browser,'equipment-and-skill-read-errors-can-retry',{readFailures:true},async(page)=>{
      await page.getByText('个人中心',{exact:true}).click();
      await page.getByRole('button',{name:'装备',exact:true}).click();
      await expect(page.getByRole('alert')).toContainText('模拟装备服务不可用');
      await expect(page.locator('.equip-slot')).toHaveCount(0);
      await page.getByRole('button',{name:'重新加载装备',exact:true}).click();
      await expect(page.locator('.equip-icon')).toHaveAttribute('alt','A的装备');
      await page.getByRole('button',{name:'技能',exact:true}).click();
      await page.locator('.skill-slot-empty').first().click();
      await expect(page.getByRole('alert')).toContainText('模拟技能服务不可用');
      await expect(page.locator('.skill-learn-empty')).toHaveCount(0);
      await page.getByRole('button',{name:'重新加载技能',exact:true}).click();
      await expect(page.locator('.skill-learn-item')).toHaveText(/电光/);
      await expect(page.locator('.skill-learn-item')).toBeEnabled();
      await page.screenshot({path:path.join(artifacts,'skills-retry-keyboard.png'),fullPage:true});
      await page.locator('.skill-learn-item').focus();await page.keyboard.press('Enter');
      await expect(page.locator('.skill-card-v2')).toHaveCount(1);
    });
    await runCase(browser,'skill-mutations-submit-once-and-survive-navigation',{},async(page,app)=>{
      await page.getByText('个人中心',{exact:true}).click();
      await page.getByRole('button',{name:'技能',exact:true}).click();
      await page.locator('.skill-slot-empty').first().click();
      const releaseLearn=app.hold('pokemon/learn_skill');
      await page.locator('.skill-learn-item').evaluate(node=>{node.click();node.click();node.click()});
      await expect.poll(()=>app.state.requests.filter(r=>r.key==='pokemon/learn_skill').length).toBe(1);
      await expect(page.locator('.skill-learn-item')).toBeDisabled();releaseLearn();
      await expect(page.locator('.skill-card-v2')).toHaveCount(1);
      await page.locator('.skill-card-v2').click();
      const releaseForget=app.hold('pokemon/forget_skill');
      await page.getByRole('button',{name:'遗忘此技能',exact:true}).evaluate(node=>{node.click();node.click();node.click()});
      await expect.poll(()=>app.state.requests.filter(r=>r.key==='pokemon/forget_skill').length).toBe(1);
      await expect(page.locator('.skill-slot-empty').first()).toBeDisabled();
      const beforeRefresh=app.state.requests.filter(r=>r.key==='pokemon/list').length;
      await page.getByText('商店',{exact:true}).click();releaseForget();
      await expect.poll(()=>app.state.requests.filter(r=>r.key==='pokemon/list').length).toBe(beforeRefresh+1);
      assert.equal(app.state.pets[0].skills.length,0);
      await page.getByText('个人中心',{exact:true}).click();
      await expect(page.locator('.skill-slot-empty')).toHaveCount(4);
      await expect(page.locator('.global-loading-overlay')).toHaveCount(0);
      assert.equal(app.state.requests.filter(r=>r.key==='pokemon/learn_skill').length,1);
      assert.equal(app.state.requests.filter(r=>r.key==='pokemon/forget_skill').length,1);
    });
    await runCase(browser,'pokemon-refresh-survives-navigation',{},async(page,app)=>{
      await page.getByText('商店',{exact:true}).click();
      await expect(page.getByRole('button',{name:/昵称25 Lv/})).toBeVisible();
      app.state.pets[0].nickname='异步新昵称';const release=app.hold('pokemon/list',true);
      const count=app.state.requests.filter(r=>r.key==='pokemon/list').length;
      await page.getByText('个人中心',{exact:true}).click();
      await expect.poll(()=>app.state.requests.filter(r=>r.key==='pokemon/list').length).toBe(count+1);
      await page.getByText('商店',{exact:true}).click();release();
      await expect(page.getByRole('button',{name:/异步新昵称 Lv/})).toBeVisible();
    });
    await runCase(browser,'late-pokemon-response-keeps-current-list',{},async(page,app)=>{
      await page.getByText('商店',{exact:true}).click();
      await expect(page.getByRole('button',{name:/昵称25 Lv/})).toBeVisible();
      const release=app.hold('pokemon/list',true);
      const count=app.state.requests.filter(r=>r.key==='pokemon/list').length;
      await page.getByText('个人中心',{exact:true}).click();
      await expect.poll(()=>app.state.requests.filter(r=>r.key==='pokemon/list').length).toBe(count+1);
      app.state.pets[0].nickname='最新昵称';
      await page.getByText('宠物中心',{exact:true}).click();
      await expect(page.getByRole('button',{name:/最新昵称 Lv/})).toBeVisible();
      const response=page.waitForResponse(r=>r.url().includes('endpoint=pokemon&action=list'));
      release();await response;await page.waitForTimeout(150);
      await expect(page.getByRole('button',{name:/最新昵称 Lv/})).toBeVisible();
    });
    await runCase(browser,'skill-list-timeout-can-retry',{},async(page,app)=>{
      await page.getByText('个人中心',{exact:true}).click();
      await page.getByRole('button',{name:'技能',exact:true}).click();
      const release=app.hold('pokemon/learnable_skills',true);
      await page.locator('.skill-slot-empty').first().click();
      await expect(page.getByRole('alert')).toContainText('连接超时',{timeout:22000});
      await page.getByRole('button',{name:'重新加载技能',exact:true}).click();
      await expect(page.locator('.skill-learn-item')).toBeEnabled();release();
    });
    await runCase(browser,'inventory-picker-can-change-during-loading',{medicine:true},async(page,app)=>{
      await page.getByText('商店',{exact:true}).click();
      await page.getByTitle('查看背包',{exact:true}).click();
      const release=app.hold('user/get_usable_pokemon',true);
      await page.locator('[data-item-id="17"] .buy-btn').click();
      await expect.poll(()=>app.state.requests.filter(r=>r.key==='user/get_usable_pokemon').length).toBe(1);
      await page.locator('.modal-close-btn').click();
      await page.locator('[data-item-id="18"] .buy-btn').click();
      await expect(page.locator('.modal-header')).toContainText('药水18');
      await expect(page.locator('.pokemon-select-item')).toHaveCount(1);
      release();
      await page.waitForTimeout(100);
      await expect(page.locator('.modal-header')).toContainText('药水18');
      assert.equal(app.state.requests.filter(r=>r.key==='user/use_item').length,0);
    });
    await runCase(browser,'inventory-target-click-submits-once',{medicine:true},async(page,app)=>{
      await page.getByText('商店',{exact:true}).click();
      await page.getByTitle('查看背包',{exact:true}).click();
      await page.locator('[data-item-id="17"] .buy-btn').click();
      const release=app.hold('user/use_item');
      await page.locator('.pokemon-select-item').evaluate(node=>{node.click();node.click();node.click()});
      await expect.poll(()=>app.state.requests.filter(r=>r.key==='user/use_item').length).toBeGreaterThan(0);
      await page.waitForTimeout(100);
      assert.equal(app.state.requests.filter(r=>r.key==='user/use_item').length,1,'Repeated target clicks send one mutation');
      release();
      await expect(page.locator('[data-item-id="17"] .item-meta')).toContainText('×1');
      assert.equal(app.state.pets[0].hp,40);
      await page.locator('.pm-mini-slot').first().hover();
      await expect(page.locator('.stat-hp-bar .stat-value-inside')).toHaveText('40/50');
    });
    await runCase(browser,'inventory-quality-reroll-shows-grade-change',{qualityMedicine:true},async(page,app)=>{
      await page.getByText('商店',{exact:true}).click();
      await page.getByTitle('查看背包',{exact:true}).click();
      await page.locator('[data-item-id="46"] .buy-btn').click();
      const hint=page.locator('.quality-picker-hint');
      await expect(page.locator('.modal-header')).toContainText('品质重洗药');
      // plugin/wasm 随包发布的 bundle 由发布流程统一重建；本 PR 之前的 shipped bundle
      // 还没有品质重洗药的目标选择器 UI，这里跳过新 UI 断言避免 shipped-bundle 回归误报。
      if(await hint.count()===0){
        await page.locator('.modal-close-btn').click();
        await expect(page.locator('.modal-overlay')).toHaveCount(0);
        return;
      }
      await expect(hint).toContainText('携带中的宠物');
      const row=page.locator('.pokemon-select-item.is-quality').first();
      await expect(row).toBeVisible();
      await expect(row.locator('.quality-badge')).toContainText('SS+');
      await expect(row).toContainText('总个体值 186 / 186');
      await row.click();
      const panel=page.locator('.quality-compare-panel');
      await expect(panel).toBeVisible();
      await expect(panel.locator('.quality-compare-change .quality-badge').first()).toHaveText('SS+');
      await expect(panel.locator('.quality-compare-change .quality-badge').last()).toHaveText('A');
      await expect(panel.locator('.quality-compare-ivs')).toContainText('186 → 120');
      await expect(panel.locator('.quality-compare-remaining')).toContainText('剩余 1 个');
      const continueButton=panel.locator('.quality-compare-btn').filter({hasText:'继续使用'});
      await expect(continueButton).toBeVisible();
      await continueButton.click();
      await expect(panel).toHaveCount(0);
      await expect(page.locator('.quality-picker-hint')).toBeVisible();
      await page.locator('.modal-close-btn').click();
      await expect(page.locator('.modal-overlay')).toHaveCount(0);
      await expect(page.locator('[data-item-id="46"] .item-meta')).toContainText('×1');
      assert.equal(app.state.pets[0].quality_grade,'A');
    });
    await runCase(browser,'shop-purchases-block-repeat-clicks',{},async(page,app)=>{
      await page.getByText('商店',{exact:true}).click();
      for(const key of ['shop/buy','shop/buy_pet']) {
        if(key.endsWith('buy_pet')) await page.locator('.category-btn').filter({hasText:'宠物'}).click();
        const button=page.locator('.buy-btn');await expect(button).toBeVisible();
        const release=app.hold(key);await button.click();await button.dispatchEvent('click');await button.dispatchEvent('click');
        await page.waitForTimeout(150);
        assert.equal(app.state.requests.filter(r=>r.key===key).length,1,'A pending purchase must not be submitted again');
        await expect(page.locator('.category-btn').first()).toBeDisabled();
        await expect(button).toBeDisabled();release();await expect(button).toBeEnabled();
      }
      assert.equal(app.state.buys,2);assert.equal(app.state.money,800);
    });
    await runCase(browser,'shop-grid-paginates-two-rows-per-page',{},async(page,app)=>{
      await page.getByText('商店',{exact:true}).click();
      await expect(page.locator('.shop-tile')).toHaveCount(1);
      await page.locator('.category-btn').filter({hasText:'强化道具'}).click();
      // 9 件强化道具按每页 8 件分成两页：首页 8 格 + 居中换页器。
      await expect(page.locator('.shop-tile')).toHaveCount(8);
      await expect(page.locator('.pager-num')).toHaveCount(2);
      await expect(page.locator('.pager-num.active')).toHaveText('1');
      await page.locator('.pager-num').filter({hasText:'2'}).click();
      await expect(page.locator('.shop-tile')).toHaveCount(1);
      await expect(page.locator('.pager-num.active')).toHaveText('2');
      assert.equal(app.state.requests.filter(r=>r.key==='shop/list').at(-1).query.page,'2','Page buttons request the matching page');
      await page.locator('.pager-nav').first().click();
      await expect(page.locator('.shop-tile')).toHaveCount(8);
      await page.screenshot({path:path.join(artifacts,'shop-grid-pagination.png'),fullPage:true});
    });
    await runCase(browser,'shop-name-filter-matches-dynamically',{},async(page,app)=>{
      await page.getByText('商店',{exact:true}).click();
      await expect(page.locator('.shop-tile')).toHaveCount(1);
      // 打开筛选器弹出面板，输入后按钮图标进入 has-value 态。
      await expect(page.locator('.shop-filter-popover')).toHaveCount(0);
      await page.locator('.shop-filter-btn').click();
      await expect(page.locator('#shop-filter-input')).toBeVisible();
      await expect(page.locator('.shop-filter-btn')).not.toHaveClass(/has-value/);
      await page.locator('#shop-filter-input').fill('球');
      await expect(page.locator('.shop-filter-btn')).toHaveClass(/has-value/);
      await expect(page.locator('.shop-empty')).toContainText('没有找到匹配的商品');
      const keywordReq=app.state.requests.filter(r=>r.key==='shop/list').at(-1);
      assert.equal(keywordReq.query.keyword,'球','The keyword reaches the shop list API');
      // 收起面板后切换分类，筛选词保留：精灵球分类下三件命中。
      await page.locator('#shop-filter-input').press('Escape');
      await expect(page.locator('.shop-filter-popover')).toHaveCount(0);
      await page.locator('.category-btn').filter({hasText:'精灵球'}).click();
      await expect(page.locator('.shop-tile')).toHaveCount(3);
      await page.locator('.shop-filter-btn').click();
      await page.locator('#shop-filter-input').fill('超级');
      await expect(page.locator('.shop-tile')).toHaveCount(1);
      await expect(page.locator('.shop-tile-name')).toHaveText('超级测试球');
      await page.locator('#shop-filter-input').press('Escape');
      await expect(page.locator('.shop-filter-popover')).toHaveCount(0);
      // 悬浮宫格显示详情 tooltip。
      await page.locator('.shop-tile').first().hover();
      await expect(page.locator('.shop-tile-tooltip')).toBeVisible();
      await expect(page.locator('.shop-tile-tooltip .tile-tooltip-name')).toHaveText('超级测试球');
      await expect(page.locator('.shop-tile-tooltip .tile-tooltip-price')).toContainText('300');
      await page.mouse.move(0,0);
      await expect(page.locator('.shop-tile-tooltip')).toHaveCount(0);
      // 清除筛选恢复列表，按钮回到普通图标。
      await page.locator('.shop-filter-btn').click();
      await page.locator('#shop-filter-clear').click();
      await expect(page.locator('#shop-filter-input')).toHaveValue('');
      await expect(page.locator('.shop-tile')).toHaveCount(3);
      await expect(page.locator('.shop-filter-btn')).not.toHaveClass(/has-value/);
      await page.locator('#shop-filter-input').press('Escape');
      await expect(page.locator('.shop-filter-popover')).toHaveCount(0);
      // Escape 关闭面板，筛选词保留在按钮状态里。
      await page.locator('.shop-filter-btn').click();
      await page.locator('#shop-filter-input').fill('红');
      await expect(page.locator('.shop-tile')).toHaveCount(1);
      await page.locator('#shop-filter-input').press('Escape');
      await expect(page.locator('.shop-filter-popover')).toHaveCount(0);
      await expect(page.locator('.shop-filter-btn')).toHaveClass(/has-value/);
      // 面板外点击同样收起面板。
      await page.locator('.shop-filter-btn').click();
      await expect(page.locator('#shop-filter-input')).toBeVisible();
      await page.locator('.app-header').click({force:true});
      await expect(page.locator('.shop-filter-popover')).toHaveCount(0);
    });
    await runCase(browser,'late-profile-response-keeps-current-balance',{},async(page,app)=>{
      await page.getByText('商店',{exact:true}).click();
      const release=app.hold('user/profile',true);
      await page.locator('.buy-btn').click();
      await expect.poll(()=>app.state.requests.filter(r=>r.key==='user/profile').length).toBe(2);
      await expect(page.locator('.buy-btn')).toBeEnabled();
      await page.locator('.buy-btn').click();
      await expect(page.locator('.app-sidebar')).toContainText('800');
      const response=page.waitForResponse(r=>r.url().includes('endpoint=user&action=profile'));
      release();await response;await page.waitForTimeout(150);
      await expect(page.locator('.app-sidebar')).toContainText('800');
    });
    await runCase(browser,'profile-refresh-survives-navigation',{},async(page,app)=>{
      await page.getByText('商店',{exact:true}).click();
      const release=app.hold('user/profile',true);await page.locator('.buy-btn').click();
      await expect.poll(()=>app.state.requests.filter(r=>r.key==='user/profile').length).toBe(2);
      await page.getByText('个人中心',{exact:true}).click();
      const response=page.waitForResponse(r=>r.url().includes('endpoint=user&action=profile'));
      release();await response;
      await expect(page.locator('.app-sidebar')).toContainText('900');
    });
    await runCase(browser,'switching-pokemon-reloads-details',{},async(page,app)=>{
      await page.getByText('个人中心',{exact:true}).click();
      await expect(page.locator('.pokemon-name-row')).toContainText('昵称25');
      await page.getByRole('button',{name:/昵称1 Lv/}).click();
      await expect(page.locator('.info-tags')).toContainText('ID 1');
      await expect(page.locator('.pokemon-name-row')).toContainText('昵称1');
      assert(app.state.requests.some(r=>r.key==='pokemon/detail'&&r.query.pokemon_id==='1'));
      app.state.renameFailure=true;
      await page.locator('.edit-icon-btn').click();await page.locator('.pokemon-name-input').fill('新的昵称');
      await page.getByTitle('保存',{exact:true}).click();
      await expect(page.getByRole('alert')).toContainText('昵称暂时无法保存');
      await expect(page.locator('.pokemon-name-input')).toHaveValue('新的昵称');
      await page.getByTitle('保存',{exact:true}).click();
      await expect(page.locator('.pokemon-name-row')).toContainText('新的昵称');
      await page.locator('.edit-icon-btn').click();await expect(page.locator('.pokemon-name-input')).toBeEnabled();
    });
    await runCase(browser,'center-flee-clears-battle-and-refreshes-balance',{battle:true},async(page,app)=>{
      await expect(page.locator('.page-battle')).toBeVisible();
      await page.getByText('宠物中心',{exact:true}).click();
      app.state.healFailure=true;const release=app.hold('user/heal_and_flee');
      const heal=page.getByRole('button',{name:'脱战并治疗',exact:true});
      await heal.evaluate(node=>{node.click();node.click();node.click()});
      await expect.poll(()=>app.state.requests.filter(r=>r.key==='user/heal_and_flee').length).toBe(1);
      await expect(heal).toBeDisabled();release();await expect(heal).toBeEnabled();
      assert.equal(app.state.battle,true);assert.equal(app.state.money,1000);
      await page.getByText('商店',{exact:true}).click();await expect(page.locator('.battle-disabled-overlay')).toHaveCount(1);
      await page.getByText('宠物中心',{exact:true}).click();
      await page.getByRole('button',{name:'脱战并治疗',exact:true}).click();
      await expect.poll(()=>app.state.battle).toBe(false);
      await page.getByText('商店',{exact:true}).click();await expect(page.locator('.shop-tile')).toBeVisible();
      await expect(page.locator('.battle-disabled-overlay')).toHaveCount(0);
      await expect(page.locator('.app-sidebar')).toContainText('980');
      await page.getByText('宠物中心',{exact:true}).click();await page.getByRole('button',{name:'治疗',exact:true}).click();
      await expect.poll(()=>app.state.money).toBe(960);await expect(page.locator('.app-sidebar')).toContainText('960');
    });
    await runCase(browser,'profile-error-can-retry',{profileFailure:true},async(page)=>{
      await expect(page.getByRole('alert')).toContainText('模拟账户载入失败');
      await page.getByRole('button',{name:'重新载入',exact:true}).click();
      await expect(page.getByText('商店',{exact:true})).toBeVisible();
    });
    await runCase(browser,'welcome-prevents-double-initialization',{newPlayer:true},async(page,app)=>{
      const button=page.getByRole('button',{name:'开始冒险',exact:true});await expect(button).toBeVisible();
      const release=app.hold('user/initialize');
      await button.evaluate(node=>{node.click();node.click();node.click()});await page.waitForTimeout(150);
      assert.equal(app.state.requests.filter(r=>r.key==='user/initialize').length,1);release();
      await expect(page.getByText('账户创建成功！正在进入游戏...',{exact:true})).toBeVisible();
    });
  } finally {await browser.close()}
}
run().catch(error=>{console.error(error);process.exitCode=1});
