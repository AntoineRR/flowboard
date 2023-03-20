import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";

import { library } from '@fortawesome/fontawesome-svg-core'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'

/* import specific icons */
import { faFolderPlus, faNoteSticky, faLightbulb, faTrash, faPen, faFloppyDisk } from '@fortawesome/free-solid-svg-icons'

library.add(faFolderPlus);
library.add(faNoteSticky);
library.add(faLightbulb);
library.add(faTrash);
library.add(faPen);
library.add(faFloppyDisk);

createApp(App).component('fa-icon', FontAwesomeIcon).mount("#app");
