export const Focus = {
  mounted (el: HTMLInputElement) {
    el.focus();
    el.select();
  }
};

export default Focus;
