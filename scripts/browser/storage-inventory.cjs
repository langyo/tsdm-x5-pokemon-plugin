// Real game WASM against an isolated HTTP fixture; no live account is mutated.
const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const http = require('node:http');
const path = require('node:path');
const {chromium, expect: baseExpect} = require('@playwright/test');
const expect = baseExpect.configure({timeout: 12000});
const wasmDirectory = process.env.GAME_WASM_DIRECTORY || path.resolve(__dirname, '../../plugin/wasm');
const artifacts = path.join(__dirname, 'artifacts/storage-inventory');
const prefix = '/source/plugin/pokemon/wasm/';
const pixel = Buffer.from('R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7', 'base64');
const html = `<!doctype html><html lang="zh-CN"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Storage fixture</title>
<link rel="stylesheet" href="${prefix}game.css"><div id="main"></div><script type="module">
import init, {WebHandle} from '${prefix}_game.js';
await init({module_or_path:'${prefix}_game_bg.wasm'}); await new WebHandle().start();
</script></html>`;

function pokemon(id, site=3) {
  return {id, name:`宠物${id}`, nickname:id === 125 ? 'Faraway friend' : null,
    type_id:25, level:(id % 50) + 1, exp:20, exp_to_next_level:80,
    exp_for_current_level:0, exp_for_next_level:100, hp:50, max_hp:50, gender:0,
    is_shiny:id % 5 === 0, site, state:1, state_text:'正常', state_class:'normal', skills:[],
    base_info:{id:25,name:`宠物${id}`,type_1:id % 2 ? '水' : '火',type_2:id % 3 === 0 ? '飞行' : null}};
}

async function startFixture() {
  const state = {requests:[], unexpected:[], released:[],
    // Deliberately reversed: displayed order must come from the chosen sort.
    pokemons:[pokemon(501,1), ...Array.from({length:125},(_,i)=>pokemon(125-i,3+i%4))],
    items:Array.from({length:110},(_,i)=>({id:1001+i,type_id:i+1,item_type:1,
      name:i < 60 ? `跨页药水${i+1}` : `其他药水${i+1}`,description:'Fixture inventory item',
      image:'hp20',quantity:2,type_name:'回复药',can_use:true}))};
  state.items.push({id:1200,type_id:200,item_type:2,name:'跨页精灵球',description:'Fixture ball',image:'jlq',quantity:3,type_name:'精灵球',can_use:true});
  state.items.push({id:1201,type_id:201,item_type:1,name:"特殊药水 O'Brien %_\\ &+?#",description:'Literal search fixture',image:'hp20',quantity:1,type_name:'回复药',can_use:true});
  async function dispatch(url, body) {
    const key = `${url.searchParams.get('endpoint')}/${url.searchParams.get('action')}`;
    state.requests.push({key, body:structuredClone(body), query:Object.fromEntries(url.searchParams)});
    let data;
    switch (key) {
      case 'user/profile': data={uid:1,username:'Fixture Trainer',group_id:10,is_admin:false,is_new_player:false,money:1000,adventure_strength:100,strength_level:1,wins:0,losses:0,total_pokemons:state.pokemons.length,total_items:state.items.length,npcid:0}; break;
      case 'user/inventory_stats': data={categories:[{type_id:1,count:111},{type_id:2,count:1}]}; break;
      case 'user/badge_status': data={hidden:false,initialized:true}; break;
      case 'user/online_players': data={total:0,players:[],max_display:20}; break;
      case 'topics/list': data={topics:[],news_announcements:[],total:0}; break;
      case 'config/global': data={is_open:true,is_enable_catch:true}; break;
      case 'pokemon/list': data={pokemons:state.pokemons,total:state.pokemons.length}; break;
      case 'battle/maps': data={maps:[],total:0}; break;
      case 'battle/recover': return {status:404,result:{success:false,error:'No active battle',code:404}};
      case 'user/inventory': {
        const search=(url.searchParams.get('search') || '').trim().toLowerCase();
        const type=Number(url.searchParams.get('type'));
        const page=Number(url.searchParams.get('page') || 1);
        const matches=state.items.filter(item=>(!type || item.item_type === type) && item.name.toLowerCase().includes(search));
        data={items:matches.slice((page-1)*50,page*50),total:matches.length,page,per_page:50,total_pages:Math.ceil(matches.length/50)};
        break;
      }
      case 'pokemon/release': {
        assert.deepEqual(Object.keys(body),['id']);
        const target=state.pokemons.find(p=>p.id === body.id);
        assert(target && target.site >= 3,'Only a stored Pokemon can be released');
        state.released.push(body.id);
        state.pokemons=state.pokemons.filter(p=>p.id !== body.id);
        data=null; break;
      }
      default: throw new Error(`Unexpected API ${key}`);
    }
    return {status:200,result:{success:true,data}};
  }
  const server=http.createServer(async(request,response)=>{
    try {
      const url=new URL(request.url,'http://localhost');
      if(url.pathname === '/plugin.php' && url.searchParams.get('endpoint') === 'avatar') {
        response.writeHead(200,{'Content-Type':'image/gif'}); response.end(pixel);
      } else if(url.pathname === '/plugin.php' && url.searchParams.has('endpoint')) {
        let body=''; for await(const chunk of request) body+=chunk;
        const {status,result}=await dispatch(url,body ? JSON.parse(body) : {});
        response.writeHead(status,{'Content-Type':'application/json; charset=utf-8'}); response.end(JSON.stringify(result));
      } else if(url.pathname === '/plugin.php' || url.pathname === '/') {
        response.writeHead(200,{'Content-Type':'text/html; charset=utf-8'}); response.end(html);
      } else if(url.pathname.startsWith(prefix)) {
        const filename=path.resolve(wasmDirectory,decodeURIComponent(url.pathname.slice(prefix.length)));
        assert(filename.startsWith(path.resolve(wasmDirectory)+path.sep));
        const bytes=await fs.readFile(filename);
        response.writeHead(200,{'Content-Type':{'.wasm':'application/wasm','.js':'text/javascript','.css':'text/css'}[path.extname(filename)] || 'application/octet-stream'}); response.end(bytes);
      } else if(url.pathname.includes('/images/')) {
        response.writeHead(200,{'Content-Type':'image/gif'}); response.end(pixel);
      } else {response.writeHead(404); response.end();}
    } catch(error) {
      state.unexpected.push(String(error));
      if(!response.headersSent) response.writeHead(500,{'Content-Type':'application/json'});
      response.end(JSON.stringify({success:false,error:String(error)}));
    }
  });
  await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));
  return {state,url:`http://127.0.0.1:${server.address().port}/plugin.php?id=pokemon:pokemon`,
    close:async()=>{server.closeAllConnections(); await new Promise(resolve=>server.close(resolve));}};
}

async function withPage(browser,name,test,{mobile=false}={}) {
  const fixture=await startFixture();
  const context=await browser.newContext(mobile ? {viewport:{width:390,height:844},isMobile:true,hasTouch:true} : {viewport:{width:1440,height:1100}});
  context.setDefaultTimeout(12000);
  context.setDefaultNavigationTimeout(20000);
  const errors=[];
  await context.route('https://**/*',route=>route.fulfill({status:200,contentType:'image/gif',body:pixel}));
  await context.tracing.start({screenshots:true,snapshots:true});
  const page=await context.newPage();
  page.on('pageerror',error=>errors.push(String(error)));
  try {
    await page.goto(fixture.url);
    await page.getByText('野外冒险',{exact:true}).click();
    await expect(page.locator('.page-adventure')).toBeVisible();
    await test(page,fixture);
    assert.deepEqual(fixture.state.unexpected,[]);
    assert.deepEqual(errors,[]);
    await page.screenshot({path:path.join(artifacts,`${name}.png`),fullPage:true});
    console.log(`PASS ${name}`);
  } catch(error) {
    await page.screenshot({path:path.join(artifacts,`${name}-failure.png`),fullPage:true}).catch(()=>{});
    await fs.writeFile(path.join(artifacts,`${name}-failure.html`),await page.content());
    console.error('Fixture requests',JSON.stringify(fixture.state.requests));
    console.error('Browser errors',errors,'Fixture errors',fixture.state.unexpected);
    throw error;
  } finally {
    await context.tracing.stop({path:path.join(artifacts,`${name}-trace.zip`)});
    await context.close(); await fixture.close();
  }
}

const storageSlots=page=>page.locator('.storage-grid [data-pokemon-id]');
const displayedIds=async page=>(await storageSlots(page).evaluateAll(nodes=>nodes.map(n=>Number(n.dataset.pokemonId))));

async function run() {
  await fs.mkdir(artifacts,{recursive:true});
  const browser=await chromium.launch({headless:true});
  try {
    await withPage(browser,'storage-filters-sort-and-safe-selection',async(page,fixture)=>{
      await page.getByTitle('宠物仓库',{exact:true}).click();
      await expect(page.locator('#storage-result-count')).toHaveText('125 / 125 只');
      await expect(storageSlots(page)).toHaveCount(60);
      assert.deepEqual(await displayedIds(page),Array.from({length:60},(_,i)=>i+1));
      // 卡片在图与等级之间显示名字：无昵称时回退到种类名。
      await expect(storageSlots(page).first().locator('.pokemon-slot-name')).toHaveText('宠物1');
      await page.locator('#storage-load-more').click();
      await expect(storageSlots(page)).toHaveCount(120);
      await page.locator('#storage-load-more').click();
      await expect(storageSlots(page)).toHaveCount(125);

      await page.locator('#storage-search').fill(' FARAWAY ');
      await expect(storageSlots(page)).toHaveCount(1);
      // Shiny cards append the 闪 badge to the name row, so match the name as a prefix.
      await expect(storageSlots(page).first().locator('.pokemon-slot-name')).toContainText('Faraway friend');
      assert.deepEqual(await displayedIds(page),[125],'Search includes entries outside the initial rendered batch');
      await page.locator('#storage-clear-filters').click();
      await expect(storageSlots(page)).toHaveCount(60);

      for(const sort of ['newest','level_asc','level_desc','oldest']) {
        await page.locator('#storage-sort').selectOption(sort);
        const expected=fixture.state.pokemons.filter(p=>p.site >= 3).sort((a,b)=>
          sort === 'newest' ? b.id-a.id : sort === 'level_asc' ? a.level-b.level || a.id-b.id :
          sort === 'level_desc' ? b.level-a.level || a.id-b.id : a.id-b.id).slice(0,60).map(p=>p.id);
        await expect.poll(()=>displayedIds(page)).toEqual(expected);
      }
      await page.locator('#storage-type-filter').selectOption('飞行');
      await page.locator('#storage-min-level').fill('20');
      await page.locator('#storage-max-level').fill('40');
      await page.locator('#storage-shiny-filter').selectOption('shiny');
      await expect.poll(()=>displayedIds(page)).toEqual([30,75,120]);
      await expect(page.locator('#storage-result-count')).toHaveText('3 / 125 只');
      const resultPanel=page.locator('.storage-panel-bottom > .panel-body');
      assert((await resultPanel.boundingBox()).height >= 160,'Filters leave room for a complete Pokemon row');
      await storageSlots(page).first().scrollIntoViewIfNeeded();
      await expect(storageSlots(page).first()).toBeInViewport({ratio:0.95});
      await page.screenshot({path:path.join(artifacts,'storage-combined-filters.png'),fullPage:true});
      await page.locator('#storage-shiny-filter').selectOption('normal');
      await expect.poll(()=>displayedIds(page)).toEqual([21,24,27,33,36,39,69,72,78,81,84,87,123]);
      await page.locator('#storage-min-level').fill('41');
      await expect(page.locator('.storage-filter-error')).toHaveText('最低等级不能高于最高等级');
      await expect(storageSlots(page)).toHaveCount(0);
      await page.locator('#storage-clear-filters').click();
      await page.getByRole('button',{name:'多选',exact:true}).click();
      await page.locator('#storage-select-all').click();
      await expect(page.locator('#storage-batch-release')).toHaveText('放生 (125)');
      await expect(storageSlots(page)).toHaveCount(60);
      await page.locator('#storage-type-filter').selectOption('水');
      await expect(page.locator('#storage-batch-release')).toBeDisabled();
      await expect(page.locator('#storage-batch-release')).toHaveText('放生 (0)');
      await page.locator('#storage-search').fill('faraway');
      await page.locator('#storage-select-all').click();
      await expect(page.locator('#storage-batch-release')).toHaveText('放生 (1)');
      await page.locator('#storage-batch-release').click();
      await expect.poll(()=>fixture.state.released).toEqual([125]);
      await expect(page.locator('#storage-result-count')).toHaveText('0 / 124 只');
      assert(fixture.state.pokemons.some(p=>p.id === 2),'Previously selected hidden Pokemon is retained');
    });

    await withPage(browser,'inventory-search-across-pages',async(page,fixture)=>{
      await page.getByTitle('查看背包',{exact:true}).click();
      await expect(page.locator('.page-inventory')).toBeVisible();
      await expect(page.locator('#inventory-result-count')).toHaveText('当前分类共 111 项');
      await page.locator('#inventory-search').fill('跨页');
      await page.locator('#inventory-search-submit').click();
      await expect(page.locator('#inventory-result-count')).toHaveText('当前分类共 60 项');
      await expect(page.locator('[data-item-id]')).toHaveCount(50);
      await page.getByRole('button',{name:'下一页 →',exact:true}).click();
      await expect(page.locator('.page-info')).toHaveText('第 2 / 2 页');
      await expect(page.locator('[data-item-id]')).toHaveCount(10);
      await expect(page.locator('[data-item-id="60"]')).toBeVisible();
      await page.screenshot({path:path.join(artifacts,'inventory-search-second-page.png'),fullPage:true});
      await page.locator('#inventory-search').fill('其他药水110');
      await page.locator('#inventory-search').press('Enter');
      await expect(page.locator('#inventory-result-count')).toHaveText('当前分类共 1 项');
      await expect(page.locator('[data-item-id="110"]')).toBeVisible();
      const searchRequests=fixture.state.requests.filter(r=>r.key === 'user/inventory');
      assert.equal(searchRequests.at(-1).query.page,'1','A new search resets pagination');
      const literal="特殊药水 O'Brien %_\\ &+?#";
      await page.locator('#inventory-search').fill(literal);
      await page.locator('#inventory-search-submit').click();
      await expect(page.locator('[data-item-id="201"]')).toBeVisible();
      assert.equal(fixture.state.requests.filter(r=>r.key === 'user/inventory').at(-1).query.search,literal,'Search URL preserves UTF-8 and reserved characters');
      await page.locator('#inventory-search').fill('跨页');
      await page.locator('#inventory-search-submit').click();
      await page.locator('.category-btn').filter({hasText:'全部物品'}).click();
      await expect(page.locator('#inventory-result-count')).toHaveText('当前分类共 61 项');
      await page.locator('.category-btn').filter({hasText:'精灵球'}).click();
      await expect(page.locator('#inventory-result-count')).toHaveText('当前分类共 1 项');
      await expect(page.locator('[data-item-id="200"]')).toBeVisible();
      await page.locator('#inventory-search').fill('nothing-matches');
      await page.locator('#inventory-search').press('Enter');
      await expect(page.locator('#inventory-result-count')).toHaveText('当前分类共 0 项');
      await expect(page.locator('.shop-empty')).toContainText('没有找到匹配的物品');
      await page.locator('#inventory-search-clear').click();
      await expect(page.locator('#inventory-search')).toHaveValue('');
      await expect(page.locator('[data-item-id="200"]')).toBeVisible();
    });

    await withPage(browser,'mobile-navigation-storage-and-inventory',async(page)=>{
      const app=page.locator('.app-layout');
      assert.equal(Math.round((await app.boundingBox()).height),600,'The embedded game keeps its 600px frame');
      const navigation=page.locator('.header-nav .nav-link');
      await expect(navigation).toHaveCount(4);
      for(const link of await navigation.all()) {
        await expect(link).toBeInViewport({ratio:0.95});
        assert((await link.boundingBox()).height >= 44,'Navigation has a usable touch target');
      }
      const main=page.locator('.app-main');
      const content=await page.locator('.app-content').boundingBox();
      const sidebar=await page.locator('.app-sidebar').boundingBox();
      assert(content.width >= 300 && sidebar.width >= 300,'Main content and sidebar both use the phone width');
      assert(sidebar.y >= content.y+content.height,'Sidebar is below the content');
      assert(await main.evaluate(node=>node.scrollHeight > node.clientHeight),'Both sections are reachable by scrolling');

      await page.getByTitle('查看背包',{exact:true}).tap();
      await expect(page.locator('#inventory-result-count')).toHaveText('当前分类共 111 项');
      await page.locator('#inventory-search').fill('其他药水110');
      await page.locator('#inventory-search-submit').tap();
      await expect(page.locator('[data-item-id="110"]')).toBeVisible();
      await page.locator('#inventory-search').scrollIntoViewIfNeeded();
      await expect(page.locator('#inventory-search')).toBeInViewport({ratio:0.95});
      await page.getByTitle('宠物仓库',{exact:true}).tap();
      await page.locator('#storage-type-filter').selectOption('飞行');
      await page.locator('#storage-min-level').fill('20');
      await page.locator('#storage-max-level').fill('40');
      await page.locator('#storage-shiny-filter').selectOption('shiny');
      await expect.poll(()=>displayedIds(page)).toEqual([30,75,120]);
      await storageSlots(page).first().scrollIntoViewIfNeeded();
      await expect(storageSlots(page).first()).toBeInViewport({ratio:0.95});
      assert(await main.evaluate(node=>node.scrollWidth <= node.clientWidth+1),'Mobile controls do not require sideways scrolling');
      await expect(navigation.nth(3)).toBeInViewport({ratio:0.95});
    },{mobile:true});
  } finally {await browser.close();}
}

run().catch(error=>{console.error(error); process.exitCode=1;});
