import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";

import { library } from '@fortawesome/fontawesome-svg-core'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'

/* import specific icons */
import { faFolder, faNoteSticky, faLightbulb, faTrash, faPen, faFloppyDisk, faPlus, faEllipsis } from '@fortawesome/free-solid-svg-icons'
import { ClickOutside, Focus } from "./directives";

library.add(faFolder);
library.add(faNoteSticky);
library.add(faLightbulb);
library.add(faTrash);
library.add(faPen);
library.add(faFloppyDisk);
library.add(faPlus);
library.add(faEllipsis);

const app = createApp(App)
app.component('fa-icon', FontAwesomeIcon).mount("#app");
app.directive('click-outside', ClickOutside);
app.directive('focus', Focus);
