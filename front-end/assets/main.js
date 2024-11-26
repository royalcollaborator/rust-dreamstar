/* PLEASE DO NOT COPY AND PASTE THIS CODE. */(function(){var w=window,C='___grecaptcha_cfg',cfg=w[C]=w[C]||{},N='grecaptcha';var gr=w[N]=w[N]||{};gr.ready=gr.ready||function(f){(cfg['fns']=cfg['fns']||[]).push(f);};w['__recaptcha_api']='https://www.google.com/recaptcha/api2/';(cfg['render']=cfg['render']||[]).push('6Le_bu4pAAAAAA6U3_Mth9GsxAnMKyQn8-r6X7Wr');w['__google_recaptcha_client']=true;var d=document,po=d.createElement('script');po.type='text/javascript';po.async=true; po.charset='utf-8';var v=w.navigator,m=d.createElement('meta');m.httpEquiv='origin-trial';m.content='A/kargTFyk8MR5ueravczef/wIlTkbVk1qXQesp39nV+xNECPdLBVeYffxrM8TmZT6RArWGQVCJ0LRivD7glcAUAAACQeyJvcmlnaW4iOiJodHRwczovL2dvb2dsZS5jb206NDQzIiwiZmVhdHVyZSI6IkRpc2FibGVUaGlyZFBhcnR5U3RvcmFnZVBhcnRpdGlvbmluZzIiLCJleHBpcnkiOjE3NDIzNDIzOTksImlzU3ViZG9tYWluIjp0cnVlLCJpc1RoaXJkUGFydHkiOnRydWV9';if(v&&v.cookieDeprecationLabel){v.cookieDeprecationLabel.getValue().then(function(l){if(l!=='treatment_1.1'&&l!=='treatment_1.2'&&l!=='control_1.1'){d.head.prepend(m);}});}else{d.head.prepend(m);}po.src='https://www.gstatic.com/recaptcha/releases/EGbODne6buzpTnWrrBprcfAY/recaptcha__en.js';po.crossOrigin='anonymous';po.integrity='sha384-zYfvuq6xV6aLevocYkVfLId59jcIkDZniQX2TsTt9LIa0Tf1ORHFh4oKI1naLgGF';var e=d.querySelector('script[nonce]'),n=e&&(e['nonce']||e.getAttribute('nonce'));if(n){po.setAttribute('nonce',n);}var s=d.getElementsByTagName('script')[0];s.parentNode.insertBefore(po, s);})();
export async function recaptcha(siteKey, actionName) {
  try {
    /**
     * ReCAPCHA
     */
    await grecaptcha.ready(async function () { });
    let token = await grecaptcha.execute(siteKey, { action: actionName });
    return token;
  } catch (e) {
    console.log(e);
    return String("not");
  }
}

export async function captureVideoFrame(videoId) {
  try {
    const video = document.getElementById(videoId);
    const canvas = document.createElement('canvas');
    canvas.width = video.videoWidth;
    canvas.height = video.videoHeight;
    const ctx = canvas.getContext('2d');
    ctx.drawImage(video, 0, 0, canvas.width, canvas.height);
    const blob = await new Promise(resolve => {
      canvas.toBlob(blob => {
        resolve(blob);
      }, 'image/png');
    });
    return blob;
  } catch (e) {
    console.log(e)
    return ""
  }
}
export async function captureCanvasImg(canvasId) {
  // Todo try webp image format or be sure png is most optimal
  try {
    const canvas = document.getElementById(canvasId);
    const ctx = canvas.getContext('2d');
    const blob = await new Promise(resolve => {
      canvas.toBlob(blob => {
        resolve(blob);
      }, 'image/png');
    });
    return blob;
  } catch (e) {
    console.log(e);
    return "";
  }
}


export async function videoToImage(videoUrl) {
  try {
    // Create a video element
    const video = document.createElement('video');

    // Set the video source
    video.src = videoUrl;

    // Wait for the video to load
    await new Promise(resolve => {
      video.onloadstart = () => {
        video.play();
      };
      setTimeout(() => {
        video.onplaying = () => {
          video.pause();
          resolve();
        };
      }, 1000);
    });

    // Create a canvas element
    const canvas = document.createElement('canvas');
    const ctx = canvas.getContext('2d');

    // Set the canvas dimensions to the video dimensions
    canvas.width = video.videoWidth;
    canvas.height = video.videoHeight;

    // Draw the video frame on the canvas
    ctx.drawImage(video, 0, 0, 500, 500);

    // Get the canvas blob
    return new Promise(resolve => {
      canvas.toBlob(blob => {
        // Create an object URL for the image blob
        const imageUrl = URL.createObjectURL(blob);
        resolve(imageUrl);
      }, 'image/jpeg');
    });
  } catch (e) {
    console.log(e)
  }
}


export async function uploadFile(signedUrl, file, fileExtention) {
  // Create the upload progress element
  let wrap = document.getElementById("progress-wrap");
  wrap.style.display = "block"
  let flag = true;
  try {
    let res = await axios.put(signedUrl, file, {
      headers: {
        "Content-Type": `${fileExtention}`,
      },
      onUploadProgress: (progressEvent) => {
        const uploadPercent = parseInt(
          Math.round((progressEvent.loaded / progressEvent.total) * 100)
        );
        let progressElement = document.getElementById("progress-bar-value");
        progressElement.style.width = `${uploadPercent}%`;
        progressElement.textContent = `${uploadPercent}%`;
      },
    });
  } catch (error) {
    flag = false;
    console.error("Error uploading video:", error.message);
  } finally {
    // Remove the upload progress element
    wrap.style.display = "none"
    return flag
  }
}

export function createCanvasElement() {
  try {
    const parent = document.getElementById("canvas-wrap");
    if (!parent) throw new Error("Parent element not found");

    // Check if the canvas already exists and remove it
    const existingCanvas = document.getElementById("drawPlace");
    if (existingCanvas) {

      existingCanvas.parentNode.removeChild(existingCanvas);
    }
    const canvas = document.createElement('canvas');
    canvas.id = "drawPlace";
    canvas.width = parent.clientWidth - 10;
    canvas.height = "200"
    canvas.className = "canvas-style";

    // Append the canvas to the parent element
    parent.insertBefore(canvas, parent.firstChild);

    return "";
  } catch (e) {
    console.log(e);
    return "";
  }
}

window.addEventListener("resize", resizeCanvas);

function resizeCanvas (e) {
  const existingCanvas = document.getElementById("drawPlace");
  if (existingCanvas) {
    createCanvasElement()
  }
}

// Clipboard copy
export function copy_clipboard(str) {
  const textarea = document.createElement('textarea');
  textarea.value = str;
  document.body.appendChild(textarea);
  textarea .select();
  try {
    const successful = document.execCommand('copy');
    return true;
  } catch (err) {
    console.error('Error copying to clipboard:', err);
    return false;
  } finally {
    document.body.removeChild(textarea);
  }
}

export function register_service_worker() {
  if ('serviceWorker' in navigator) {
      window.addEventListener('load', function() {
          navigator.serviceWorker.register('/service-worker.js').then(function(registration) {
              console.log('ServiceWorker registration successful with scope: ', registration.scope);
          }, function(err) {
              console.log('ServiceWorker registration failed: ', err);
          });
      });
  }
}

let deferredPrompt;

window.addEventListener('beforeinstallprompt', (e) => {
    // Prevent the mini-infobar from appearing on mobile
    e.preventDefault();
    // Store the event so it can be triggered later
    deferredPrompt = e;

    // Show the install button (or you can trigger the install automatically)
    // document.getElementById('installButton').style.display = 'block';
});

export function installPWA() {
  if (isIOSOrMac()) {
    if (!isSafari()) {
      return "safari"
    } else {
      return "ios"
    }
  } else {
    if (deferredPrompt) {
      // Show the install prompt
      deferredPrompt.prompt();

      // Wait for the user to respond to the prompt
      deferredPrompt.userChoice.then((choiceResult) => {
          if (choiceResult.outcome === 'accepted') {
            try {
              document.getElementById('app-install').style.display = 'none';
            } catch (e){
              console.log(e);
            }
              console.log('PWA setup accepted');
          } else {
              console.log('PWA setup dismissed');
          }
          deferredPrompt = null;
      });
  }
  return "true"
  }
}

export function isPwaInstalled() {
  // For iOS (check if running in standalone mode)
  if (window.navigator.standalone) {
    return false; // Already installed on iOS
  }

  // For other browsers (check if running in PWA mode)
  if (window.matchMedia('(display-mode: standalone)').matches) {
    return false; // Already installed on Android or desktop
  }
  // If neither standalone nor PWA mode, assume it's a regular browser
  return true; // Running in a web browser, so show install button
}

// Function to check if the system is iOS or macOS
function isIOSOrMac() {
  const userAgent = navigator.userAgent;

  // Check if the device is an iOS device
  const isIOS = /iPad|iPhone|iPod/.test(userAgent) && !window.MSStream;

  // Check if the device is a macOS device
  const isMac = /Macintosh/.test(userAgent) && !isIOS; // Exclude iOS devices from macOS check
  return isIOS || isMac;
}

// Function to check if the browser is Safari
function isSafari() {
  const userAgent = navigator.userAgent;
  const isSafari = /^((?!chrome|android).)*safari/i.test(userAgent);
  return isSafari;
}