// Turns each license entry into a real disclosure widget.
//
// The page used to be built from <details><summary><h4>crate names</h4></summary>, which reads
// well until you check the state: NVDA announced "collapsed" whether the section was open or not,
// because the state lives on the summary and the heading you navigate to is a child of it. A
// button carrying aria-expanded is announced correctly by every screen reader, and NVDA picks up
// the change the moment the attribute flips.
//
// The markup ships expanded, so a visitor with no JavaScript gets a long page rather than an
// unopenable one. Collapsing happens here, once, on load.
(() => {
	const entries = document.querySelectorAll('.licenses .license');
	entries.forEach((entry, index) => {
		const heading = entry.querySelector('.license-name');
		const body = entry.querySelector('.license-body');
		if (!heading || !body) return;
		const id = `license-body-${index}`;
		body.id = id;
		body.hidden = true;
		const button = document.createElement('button');
		button.type = 'button';
		button.className = 'disclosure';
		button.setAttribute('aria-expanded', 'false');
		button.setAttribute('aria-controls', id);
		button.textContent = heading.textContent;
		heading.textContent = '';
		heading.appendChild(button);
		button.addEventListener('click', () => {
			const open = button.getAttribute('aria-expanded') === 'true';
			button.setAttribute('aria-expanded', open ? 'false' : 'true');
			body.hidden = open;
		});
	});
})();
