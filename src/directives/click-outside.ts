function handleOutsideClick(e: Event, el: HTMLElement, handler: () => void, include: () => Node[]) {
  // Elements with the returned by the "include" call will not trigger the handler
  const included = include();
  if (e.target instanceof Node && !el.contains(e.target) && !included.some(inc => inc.contains(e.target as Node))) {
    handler();
  }
}

let eventListener: (e: Event) => void;

export const ClickOutside = {
  mounted (el: HTMLElement, binding: any) {
    const { handler, include } = binding.value;
    eventListener = (e: Event) => handleOutsideClick(e, el, handler, include);
    document.addEventListener('click', eventListener, true);
    document.addEventListener('touchstart', eventListener, true);
  },
  
  unmounted (el: HTMLElement, binding: any) {
    document.removeEventListener('click', eventListener, true);
    document.removeEventListener('touchstart', eventListener, true);
  },
};

export default ClickOutside;
