<?php
echo '<script>
(function(){
var uids=[];document.querySelectorAll(".pls.favatar a[href*=\"uid=\"]").forEach(function(a){var m=a.href.match(/uid=(\d+)/);if(m)uids.push(m[1])});
if(!uids.length)return;
var x=new XMLHttpRequest();x.open("GET","plugin.php?id=pokemon:pokemon&endpoint=badges&uids="+uids.join(","));
x.onload=function(){try{var d=JSON.parse(x.responseText);if(!d.success)return;
for(var uid in d.data){var h=d.data[uid];if(!h)continue;var el=document.querySelector(".pls.favatar a[href*=\"uid="+uid+"\"]");if(el){var div=document.createElement("div");div.style.cssText="text-align:center;padding:6px 0;border-bottom:1px dashed #eee";div.innerHTML=h;el.closest(".pls").appendChild(div)}}
}catch(e){}};x.send();
})();
</script>';
