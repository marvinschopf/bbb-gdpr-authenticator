document.addEventListener("DOMContentLoaded", ()=>{
  document.getElementById("yes").addEventListener("click", ()=>{
    // cookie for one week
    document.cookie = "cookie_consented=true;max-age=604800;path=/;domain={{DOMAIN}};SameSite=Lax;Secure";
    let target_url = null;
    try {
      target_url = window.location.search.split("&").map(window.decodeURIComponent).map(s=>s.match(/\?url=(.*)/)).filter(Array.isArray)[0][1];
      console.log("found target url " + target_url);
    } catch (e) {
      if (e instanceof TypeError) {
        console.log("no target url specified")
      } else {
        throw e;
      }
    }

    if (target_url && target_url.startsWith(window.location.origin)) {
      console.log("autorized url, redirecting");
      window.location.replace(target_url);
    } else {
      console.log("redirecting to homepage");
      window.location.replace("/");
    }
  })
})
