import counter from 'mu/function/counter';

export function setupCounter(element) {
  const updateCounter = async () => {
    const count = await counter.count();
    element.innerHTML = `count is ${count}`;
  }
  element.addEventListener('click', () => updateCounter());
  updateCounter();
}