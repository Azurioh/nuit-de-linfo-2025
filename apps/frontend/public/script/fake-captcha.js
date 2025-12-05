// import "../../src/styles/captcha.css";

(() => {
	const root = document.getElementById("fake-captcha-root");
	if (!root) return;

	const steps = [];
	let currentStep = 0;

	function fadeOutIn(callback) {
		root.style.transition = "opacity 0.5s";
		root.style.opacity = "0";
		setTimeout(() => {
			callback();
			root.style.opacity = "0";
			setTimeout(() => {
				root.style.opacity = "1";
			}, 30);
		}, 500);
	}

	function nextStep() {
		if (currentStep < steps.length) {
			fadeOutIn(() => steps[currentStep++]());
		}
	}

	steps.push(() => {
		root.innerHTML = `
    <div class="captcha-card">
      <h1>Visez les cibles !</h1>
      <p>Cliquez sur 8 cibles avant qu'elles disparaissent</p>
      <div id="targetArea" style="position:relative;width:100%;height:300px;background:#1a1a1a;border-radius:12px;margin-top:20px;overflow:hidden;"></div>
      <p id="targetScore" style="margin-top:15px;font-size:20px;font-weight:bold;">0 / 8</p>
    </div>
  `;

		const area = document.getElementById("targetArea");
		const scoreDisplay = document.getElementById("targetScore");
		let hits = 0;

		function spawnTarget() {
			if (hits >= 8) return;

			const target = document.createElement("div");
			target.className = "popup-target";
			target.style.left = `${Math.random() * (area.clientWidth - 60)}px`;
			target.style.top = `${Math.random() * (area.clientHeight - 60)}px`;
			area.appendChild(target);

			const timeout = setTimeout(() => {
				target.remove();
				if (hits < 8) spawnTarget();
			}, 1500);

			target.addEventListener("click", () => {
				clearTimeout(timeout);
				hits++;
				scoreDisplay.textContent = `${hits} / 8`;
				target.style.background = "#2a9d8f";
				target.style.transform = "scale(1.3)";
				setTimeout(() => target.remove(), 200);

				if (hits >= 8) {
					setTimeout(nextStep, 500);
				} else {
					setTimeout(spawnTarget, 400);
				}
			});
		}

		spawnTarget();
	});

	steps.push(() => {
		const fakeText = `CONDITIONS GÉNÉRALES D'UTILISATION

Article 1 - Dispositions générales
Les présentes Conditions Générales d'Utilisation (ci-après "CGU") régissent l'utilisation de ce faux CAPTCHA totalement inutile. En utilisant ce service, vous acceptez sans réserve les présentes CGU.

Article 2 - Objet
Ce CAPTCHA n'a strictement aucune utilité. Il ne vérifie rien et ne protège contre aucun bot. Son seul objectif est de vous faire perdre votre temps de manière ludique.

Article 3 - Obligations de l'utilisateur
L'utilisateur s'engage à :
- Compléter toutes les épreuves absurdes proposées
- Ne pas râler contre la longueur excessive de ces CGU
- Accepter que tout ceci est parfaitement inutile
- Reconnaître qu'il aurait pu faire quelque chose de productif à la place

Article 4 - Données personnelles
Nous ne collectons aucune donnée. Vraiment. Ce serait trop d'efforts pour un projet aussi futile.

Article 5 - Propriété intellectuelle
Tous les mini-jeux ridicules contenus dans ce CAPTCHA sont notre propriété exclusive. Toute reproduction est autorisée car franchement, qui voudrait copier ça ?

Article 6 - Responsabilité
Nous déclinons toute responsabilité concernant :
- Le temps perdu à compléter ce CAPTCHA
- La frustration engendrée par certaines épreuves
- L'impression d'avoir fait quelque chose d'inutile
- Les dommages psychologiques causés par le puzzle taquin

Article 7 - Durée et résiliation
Ces CGU s'appliquent tant que vous êtes sur cette page. Vous pouvez les résilier à tout moment en fermant l'onglet.

Article 8 - Modifications
Nous nous réservons le droit de modifier ces CGU à tout moment, sans préavis, juste pour embêter les gens qui les lisent vraiment.

Article 9 - Litiges
En cas de litige, bonne chance pour nous retrouver.

Article 10 - Loi applicable
Ces CGU sont régies par les lois de l'absurde et du temps perdu.

`.repeat(3);

		root.innerHTML = `
    <div class="captcha-card" style="max-width:600px;">
      <h1>Conditions Générales d'Utilisation</h1>
      <p>Veuillez lire attentivement et faire défiler jusqu'en bas</p>
      <div id="cguScroll" class="cgu-scroll">${fakeText}</div>
      <label style="display:flex;align-items:center;justify-content:center;margin-top:20px;opacity:0.3;" id="cguCheckLabel">
        <input type="checkbox" id="cguCheck" disabled style="width:auto;margin-right:10px;">
        <span>J'ai lu et j'accepte les CGU (faites défiler en bas)</span>
      </label>
      <button id="cguBtn" disabled style="margin-top:15px;opacity:0.3;">Continuer</button>
    </div>
  `;

		const scroll = document.getElementById("cguScroll");
		const checkbox = document.getElementById("cguCheck");
		const checkLabel = document.getElementById("cguCheckLabel");
		const btn = document.getElementById("cguBtn");

		scroll.addEventListener("scroll", () => {
			const isBottom =
				scroll.scrollHeight - scroll.scrollTop <= scroll.clientHeight + 10;
			if (isBottom) {
				checkbox.disabled = false;
				checkLabel.style.opacity = "1";
			}
		});

		checkbox.addEventListener("change", () => {
			if (checkbox.checked) {
				btn.disabled = false;
				btn.style.opacity = "1";
			}
		});

		btn.addEventListener("click", nextStep);
	});

	steps.push(() => {
		root.innerHTML = `
    <div class="captcha-card">
      <h1>Connectez les points</h1>
      <p>Cliquez sur les numéros dans l'ordre (1 → 10)</p>
      <div id="dotsArea" style="position:relative;width:100%;height:350px;background:#1a1a1a;border-radius:12px;margin-top:20px;">
        <canvas id="dotsCanvas" width="540" height="350" style="position:absolute;top:0;left:0;pointer-events:none;"></canvas>
      </div>
    </div>
  `;

		const area = document.getElementById("dotsArea");
		const canvas = document.getElementById("dotsCanvas");
		const ctx = canvas.getContext("2d");

		const dots = [];
		for (let i = 1; i <= 10; i++) {
			const x = Math.random() * 440 + 50;
			const y = Math.random() * 250 + 50;
			dots.push({ num: i, x, y });
		}

		let current = 1;
		const lines = [];

		dots.forEach((dot) => {
			const el = document.createElement("div");
			el.className = "connect-dot";
			el.textContent = dot.num;
			el.style.left = `${dot.x - 20}px`;
			el.style.top = `${dot.y - 20}px`;

			el.addEventListener("click", () => {
				if (dot.num === current) {
					el.classList.add("connected");

					if (current > 1) {
						const prev = dots.find((d) => d.num === current - 1);
						lines.push({ x1: prev.x, y1: prev.y, x2: dot.x, y2: dot.y });
						drawLines();
					}

					current++;
					if (current > 10) {
						setTimeout(nextStep, 500);
					}
				} else {
					el.classList.add("shake");
					setTimeout(() => el.classList.remove("shake"), 300);
				}
			});

			area.appendChild(el);
		});

		function drawLines() {
			ctx.clearRect(0, 0, 540, 350);
			ctx.strokeStyle = "#e9c46a";
			ctx.lineWidth = 3;
			lines.forEach((line) => {
				ctx.beginPath();
				ctx.moveTo(line.x1, line.y1);
				ctx.lineTo(line.x2, line.y2);
				ctx.stroke();
			});
		}
	});

	steps.push(() => {
		root.innerHTML = `
      <div class="captcha-card">
        <h1>Mini-jeu clavier</h1>
        <p>Tapez la séquence suivante sans erreur :</p>
        <div style="font-size:32px;letter-spacing:10px;font-weight:bold;margin:24px 0;color:#e9c46a;">A S D A S D</div>
        <input id="patternInput" type="text" maxlength="6" style="width:220px;text-transform:uppercase;">
        <p id="patternError" class="error-message"></p>
      </div>
    `;
		const input = document.getElementById("patternInput");
		const error = document.getElementById("patternError");
		input.focus();
		input.addEventListener("input", () => {
			const val = input.value.toUpperCase();
			input.value = val;
			if ("ASDASD".startsWith(val)) {
				error.textContent = "";
				if (val === "ASDASD") nextStep();
			} else {
				error.textContent = "❌ Mauvaise séquence, recommencez !";
				input.value = "";
			}
		});
	});

	// --- Step 14: Drag & drop emoji ordering ---
	steps.push(() => {
		const ordered = ["🐸", "🐱", "🐻", "🦁", "🐼"];
		const shuffled = [...ordered].sort(() => Math.random() - 0.5);
		root.innerHTML = `
      <div class="captcha-card">
        <h1>Ordre des emojis</h1>
        <p>Glissez-déposez pour remettre les emojis dans l'ordre suivant :</p>
        <div style="font-size:22px;margin:12px 0 24px 0;">🐸 &nbsp; 🐱 &nbsp; 🐻 &nbsp; 🦁 &nbsp; 🐼</div>
        <div id="emojiOrder" style="display:flex;gap:18px;justify-content:center;user-select:none;"></div>
        <p class="hint-text">Faites glisser un emoji sur un autre pour les échanger.</p>
      </div>
    `;
		const container = document.getElementById("emojiOrder");
		const arr = [...shuffled];
		function render() {
			container.innerHTML = "";
			arr.forEach((em, i) => {
				const div = document.createElement("div");
				div.textContent = em;
				div.className = "emoji-large";
				div.style.background = "#222";
				div.style.cursor = "grab";
				div.setAttribute("draggable", "true");
				div.setAttribute("data-idx", i);
				div.addEventListener("dragstart", (e) => {
					e.dataTransfer.setData("text/plain", i);
					div.style.opacity = "0.3";
				});
				div.addEventListener("dragend", () => {
					div.style.opacity = "1";
				});
				div.addEventListener("dragover", (e) => e.preventDefault());
				div.addEventListener("drop", (e) => {
					e.preventDefault();
					const from = parseInt(e.dataTransfer.getData("text/plain"), 10);
					const to = i;
					[arr[from], arr[to]] = [arr[to], arr[from]];
					render();
					check();
				});
				container.appendChild(div);
			});
		}
		function check() {
			if (arr.join() === ordered.join()) {
				setTimeout(nextStep, 500);
			}
		}
		render();
	});

	steps.push(() => {
		const num1 = Math.floor(Math.random() * 20) + 5;
		const num2 = Math.floor(Math.random() * 20) + 5;
		const answer = num1 + num2;

		root.innerHTML = `
      <div class="captcha-card">
        <h1>Prouvez que vous savez compter</h1>
        <p style="font-size:24px;margin:20px 0;">Combien font ${num1} + ${num2} ?</p>
        <input id="mathInput" type="number" style="width:200px;" placeholder="Votre réponse">
        <button id="submitMath">Valider</button>
        <p id="mathError" class="error-message"></p>
      </div>
    `;

		const input = document.getElementById("mathInput");
		const btn = document.getElementById("submitMath");
		const error = document.getElementById("mathError");

		function check() {
			if (parseInt(input.value, 10) === answer) {
				root.style.opacity = "0";
				setTimeout(() => {
					root.style.transition = "opacity 0.5s";
					root.style.opacity = "1";
					nextStep();
				}, 30);
			} else {
				error.textContent = "❌ Mauvaise réponse, essayez encore !";
				input.value = "";
				input.focus();
			}
		}

		btn.addEventListener("click", check);
		input.addEventListener("keypress", (e) => {
			if (e.key === "Enter") check();
		});
		input.focus();
	});

	steps.push(() => {
		const animals = [
			"🐶",
			"🐱",
			"🐭",
			"🐹",
			"🐰",
			"🦊",
			"🐻",
			"🐼",
			"🐨",
			"🐯",
			"🦁",
			"🐮",
			"🐷",
			"🐸",
			"🐵",
			"🚗",
			"🐔",
			"🐧",
			"🐦",
			"🐤",
		];
		const shuffled = animals.sort(() => Math.random() - 0.5);

		root.innerHTML = `
      <div class="captcha-card">
        <h1>Trouvez l'intrus !</h1>
        <p>Un de ces éléments n'est pas un animal...</p>
        <div id="intrusGrid" class="grid-container" style="grid-template-columns:repeat(5, 1fr);max-width:400px;"></div>
      </div>
    `;

		const grid = document.getElementById("intrusGrid");
		shuffled.forEach((item) => {
			const div = document.createElement("div");
			div.textContent = item;
			div.className = "grid-item";
			div.style.fontSize = "32px";
			div.style.border = "2px solid #ccc";

			div.addEventListener("click", () => {
				if (item === "🚗") {
					nextStep();
				} else {
					div.style.background = "#ffe0e0";
					div.classList.add("shake");
					setTimeout(() => {
						div.style.background = "";
						div.classList.remove("shake");
					}, 300);
				}
			});

			grid.appendChild(div);
		});
	});

	steps.push(() => {
		let clicks = 0;
		root.innerHTML = `
      <div class="captcha-card">
        <h1>Cliquez exactement 10 fois</h1>
        <p style="font-size:48px;margin:30px 0;font-weight:bold;" id="clickCount">0</p>
        <button id="clickBtn" style="padding:20px 40px;font-size:24px;">Cliquer</button>
        <p class="hint-text">Ni plus, ni moins !</p>
      </div>
    `;

		const count = document.getElementById("clickCount");
		const btn = document.getElementById("clickBtn");

		btn.addEventListener("click", () => {
			clicks++;
			count.textContent = clicks;

			if (clicks === 10) {
				setTimeout(nextStep, 500);
			} else if (clicks > 10) {
				count.style.color = "#e63946";
				count.textContent = "Raté ! 😢";
				setTimeout(() => {
					clicks = 0;
					count.textContent = "0";
					count.style.color = "";
				}, 1000);
			}
		});
	});

	steps.push(() => {
		root.innerHTML = `
      <div class="captcha-card">
        <h1>Créez la couleur verte</h1>
        <p>Ajustez les curseurs RGB</p>
        <div style="margin:30px 0;">
          <div style="margin:15px 0;">
            <label>Rouge: <span id="rVal">128</span></label><br>
            <input type="range" id="rSlider" min="0" max="255" value="128">
          </div>
          <div style="margin:15px 0;">
            <label>Vert: <span id="gVal">128</span></label><br>
            <input type="range" id="gSlider" min="0" max="255" value="128">
          </div>
          <div style="margin:15px 0;">
            <label>Bleu: <span id="bVal">128</span></label><br>
            <input type="range" id="bSlider" min="0" max="255" value="128">
          </div>
        </div>
        <div id="colorPreview" class="color-preview" style="background:rgb(128,128,128);"></div>
      </div>
    `;

		const rSlider = document.getElementById("rSlider");
		const gSlider = document.getElementById("gSlider");
		const bSlider = document.getElementById("bSlider");
		const preview = document.getElementById("colorPreview");

		function update() {
			const r = parseInt(rSlider.value, 10);
			const g = parseInt(gSlider.value, 10);
			const b = parseInt(bSlider.value, 10);

			document.getElementById("rVal").textContent = r;
			document.getElementById("gVal").textContent = g;
			document.getElementById("bVal").textContent = b;

			preview.style.background = `rgb(${r},${g},${b})`;

			if (r < 50 && g > 200 && b < 50) {
				setTimeout(nextStep, 500);
			}
		}

		rSlider.addEventListener("input", update);
		gSlider.addEventListener("input", update);
		bSlider.addEventListener("input", update);
	});

	steps.push(() => {
		const numbers = [7, 2, 9, 1, 5, 3, 8, 4, 6];
		const shuffled = [...numbers].sort(() => Math.random() - 0.5);

		root.innerHTML = `
      <div class="captcha-card">
        <h1>Triez les nombres par ordre croissant</h1>
        <p>Cliquez pour échanger deux nombres adjacents</p>
        <div id="sortGrid" style="display:flex;gap:10px;margin:30px 0;justify-content:center;flex-wrap:wrap;max-width:400px;"></div>
      </div>
    `;

		const grid = document.getElementById("sortGrid");
		const arr = [...shuffled];

		function render() {
			grid.innerHTML = "";
			arr.forEach((num, i) => {
				const div = document.createElement("div");
				div.textContent = num;
				div.style.fontSize = "28px";
				div.style.padding = "16px 20px";
				div.style.background = "#111";
				div.style.color = "white";
				div.style.borderRadius = "8px";
				div.style.cursor = "pointer";
				div.style.userSelect = "none";

				div.addEventListener("click", () => {
					if (i < arr.length - 1) {
						[arr[i], arr[i + 1]] = [arr[i + 1], arr[i]];
						render();
						check();
					}
				});

				grid.appendChild(div);
			});
		}

		function check() {
			const sorted = arr.every((val, i) => i === 0 || arr[i - 1] <= val);
			if (sorted) {
				setTimeout(nextStep, 500);
			}
		}

		render();
	});

	steps.push(() => {
		root.innerHTML = `
      <div class="captcha-card">
        <h1>Faites glisser pour confirmer</h1>
        <div id="sliderContainer" class="slider-container">
          <div id="sliderTrack" style="position:absolute;top:50%;left:0;width:100%;height:10px;background:linear-gradient(90deg, #bbb 25%, #eee 50%, #bbb 75%);border-radius:5px;transform:translateY(-50%);"></div>
          <div id="slider" class="slider-knob"></div>
        </div>
      </div>
    `;
		const slider = document.getElementById("slider");
		const container = document.getElementById("sliderContainer");
		const track = document.getElementById("sliderTrack");

		const waveSVG = `
      <svg width="100%" height="10" viewBox="0 0 100 10" preserveAspectRatio="none" xmlns="http://www.w3.org/2000/svg">
        <path fill="#ddd" d="M0 5 Q 12.5 0 25 5 T 50 5 T 75 5 T 100 5 V10 H0 Z" />
      </svg>
    `;
		const svgBase64 = `data:image/svg+xml;base64,${btoa(waveSVG)}`;
		track.style.background = `url("${svgBase64}") repeat-x`;
		track.style.backgroundSize = "50px 10px";

		const amplitude = 5;
		const wavelength = 50;
		const knobWidth = slider.offsetWidth;

		function getY(x) {
			const center = container.clientHeight / 2;
			const y = center + amplitude * Math.sin((2 * Math.PI * x) / wavelength);
			return y;
		}

		slider.style.left = "0px";
		slider.style.top = `${getY(0) - knobWidth / 2}px`;

		let dragging = false;

		function onDrag(x) {
			const rect = container.getBoundingClientRect();
			const clampedX = Math.max(0, Math.min(rect.width - knobWidth, x));
			const y = getY(clampedX + knobWidth / 2);
			slider.style.left = `${clampedX}px`;
			slider.style.top = `${y - knobWidth / 2}px`;

			if (dragging && clampedX >= rect.width - knobWidth) {
				dragging = false;
				nextStep();
			}
		}

		slider.addEventListener("mousedown", (e) => {
			e.preventDefault();
			dragging = true;
		});
		slider.addEventListener(
			"touchstart",
			(e) => {
				e.preventDefault();
				dragging = true;
			},
			{ passive: false },
		);

		document.addEventListener("mouseup", () => {
			dragging = false;
		});
		document.addEventListener("touchend", () => {
			dragging = false;
		});

		document.addEventListener("mousemove", (e) => {
			if (!dragging) return;
			const rect = container.getBoundingClientRect();
			onDrag(e.clientX - rect.left - knobWidth / 2);
		});

		document.addEventListener(
			"touchmove",
			(e) => {
				if (!dragging) return;
				if (e.touches.length === 0) return;
				const touch = e.touches[0];
				const rect = container.getBoundingClientRect();
				onDrag(touch.clientX - rect.left - knobWidth / 2);
			},
			{ passive: false },
		);
	});

	steps.push(() => {
		const colors = ["#e63946", "#457b9d", "#2a9d8f", "#e9c46a"];
		const sequence = [];
		let playerSequence = [];
		let round = 0;

		root.innerHTML = `
      <div class="captcha-card">
        <h1>Jeu de mémoire</h1>
        <p id="status">Regardez la séquence...</p>
        <div id="memoryGrid" style="display:grid;grid-template-columns:repeat(2, 120px);gap:15px;margin:30px auto;width:fit-content;"></div>
      </div>
    `;

		const grid = document.getElementById("memoryGrid");
		const status = document.getElementById("status");

		colors.forEach((color, i) => {
			const div = document.createElement("div");
			div.className = "memory-box";
			div.style.background = color;
			div.dataset.index = i;

			div.addEventListener("click", () => {
				if (sequence.length === 0) return;

				div.classList.add("active");
				setTimeout(() => {
					div.classList.remove("active");
				}, 200);

				playerSequence.push(i);

				if (
					playerSequence[playerSequence.length - 1] !==
					sequence[playerSequence.length - 1]
				) {
					status.textContent = "❌ Raté ! On recommence...";
					setTimeout(startRound, 1500);
					return;
				}

				if (playerSequence.length === sequence.length) {
					round++;
					if (round >= 3) {
						status.textContent = "🎉 Bravo !";
						setTimeout(nextStep, 1000);
					} else {
						status.textContent = "✅ Bien ! Prochain niveau...";
						setTimeout(startRound, 1500);
					}
				}
			});

			grid.appendChild(div);
		});

		function startRound() {
			playerSequence = [];
			sequence.push(Math.floor(Math.random() * 4));
			status.textContent = `Niveau ${round + 1} - Regardez...`;

			let i = 0;
			const interval = setInterval(() => {
				if (i >= sequence.length) {
					clearInterval(interval);
					status.textContent = "À vous de jouer !";
					return;
				}

				const idx = sequence[i];
				const div = grid.children[idx];
				div.classList.add("active");
				setTimeout(() => {
					div.classList.remove("active");
				}, 400);

				i++;
			}, 800);
		}

		setTimeout(startRound, 1000);
	});

	steps.push(() => {
		root.innerHTML = `
      <div class="captcha-card">
        <h1>Question piège</h1>
        <p style="font-size:20px;margin:30px 0;max-width:500px;">
          Si un avion s'écrase exactement sur la frontière entre la France et l'Espagne,
          dans quel pays enterre-t-on les survivants ?
        </p>
        <input id="trickInput" type="text" style="width:300px;" placeholder="Votre réponse">
        <button id="submitTrick">Valider</button>
        <p id="trickHint" class="hint-text"></p>
      </div>
    `;

		const input = document.getElementById("trickInput");
		const btn = document.getElementById("submitTrick");
		const hint = document.getElementById("trickHint");
		let attempts = 0;

		function check() {
			const answer = input.value.toLowerCase().trim();

			if (
				answer.includes("survivant") ||
				answer.includes("nulle") ||
				answer.includes("pas")
			) {
				nextStep();
			} else {
				attempts++;
				if (attempts === 1) {
					hint.textContent = "💡 Indice : Lisez bien la question...";
				} else if (attempts === 2) {
					hint.textContent = "💡 Les SURVIVANTS...";
				} else {
					hint.textContent =
						"😅 On n'enterre pas les survivants ! Tapez 'nulle part' pour continuer.";
				}
				input.value = "";
			}
		}

		btn.addEventListener("click", check);
		input.addEventListener("keypress", (e) => {
			if (e.key === "Enter") check();
		});
		input.focus();
	});

	steps.push(() => {
		root.innerHTML = `
      <div class="captcha-card">
        <h1>Test de frappe</h1>
        <p>Tapez le mot suivant sans erreur :</p>
        <p style="font-size:26px;font-weight:bold;margin:20px 0;">anticonstitutionnellement</p>
        <input id="typeInput" type="text" style="width:350px;">
        <p id="typeError" class="error-message"></p>
      </div>
    `;
		const input = document.getElementById("typeInput");
		const error = document.getElementById("typeError");
		input.addEventListener("input", () => {
			if (input.value === "anticonstitutionnellement") nextStep();
			else if (!"anticonstitutionnellement".startsWith(input.value))
				error.textContent = "❌ Faux, recommencez.";
			else error.textContent = "";
		});
	});

	steps.push(() => {
		const emojis = ["😀", "😃", "😄", "😁", "😆", "🙂", "😊"];
		const shuffled = emojis.sort(() => Math.random() - 0.5);

		root.innerHTML = `
      <div class="captcha-card">
        <h1>Trouvez 😀</h1>
        <div id="emojiGrid" style="display:grid;grid-template-columns:repeat(7,60px);gap:10px;margin-top:20px;"></div>
      </div>
    `;

		const grid = document.getElementById("emojiGrid");
		shuffled.forEach((e) => {
			const div = document.createElement("div");
			div.textContent = e;
			div.style.fontSize = "32px";
			div.style.padding = "10px";
			div.style.cursor = "pointer";
			div.style.textAlign = "center";
			div.addEventListener("click", () => {
				if (e === "😀") nextStep();
				else div.style.opacity = "0.3";
			});
			grid.appendChild(div);
		});
	});

	steps.push(() => {
		root.innerHTML = `
      <div class="captcha-card">
        <button id="skipTaquinBtn" class="btn-danger" style="display:block;margin-bottom:20px;width:fit-content;">⚠️ PASSER LE TAQUIN (temporaire)</button>
        <h1>Puzzle taquin</h1>
        <p>Remettez les chiffres en ordre</p>
        <div id="taquin" style="display:grid;grid-template-columns:repeat(3,80px);gap:8px;margin-top:20px;"></div>
      </div>
    `;
		document.getElementById("skipTaquinBtn").addEventListener("click", () => {
			console.warn("⚠️ Puzzle Taquin skipped (temporary)");
			nextStep();
		});
		const taquin = document.getElementById("taquin");
		const tiles = [1, 2, 3, 4, 5, 6, 7, 8, null].sort(
			() => Math.random() - 0.5,
		);

		function render() {
			taquin.innerHTML = "";
			tiles.forEach((v, i) => {
				const div = document.createElement("div");
				div.style.height = "80px";
				div.style.display = "flex";
				div.style.alignItems = "center";
				div.style.justifyContent = "center";
				div.style.fontSize = "28px";
				div.style.fontWeight = "bold";
				div.style.background = v ? "#111" : "transparent";
				div.style.color = "white";
				div.textContent = v ? v : "";
				div.style.borderRadius = "8px";
				if (v) {
					div.style.cursor = "pointer";
					div.addEventListener("click", () => {
						const empty = tiles.indexOf(null);
						if ([i - 1, i + 1, i - 3, i + 3].includes(empty)) {
							[tiles[i], tiles[empty]] = [tiles[empty], tiles[i]];
							render();
							if (tiles.join() === "1,2,3,4,5,6,7,8,") nextStep();
						}
					});
				}
				taquin.appendChild(div);
			});
		}
		render();
	});

	steps.push(() => {
		const secret = Math.floor(Math.random() * 50) + 1;
		root.innerHTML = `
      <div class="captcha-card">
        <h1>Devinez le nombre (1 à 50)</h1>
        <input id="guessInput" type="number" style="width:200px;">
        <button id="guessBtn" style="margin-left:10px;">OK</button>
        <p id="guessHint" style="margin-top:15px;"></p>
      </div>
    `;
		const input = document.getElementById("guessInput");
		const hint = document.getElementById("guessHint");

		document.getElementById("guessBtn").addEventListener("click", () => {
			const v = parseInt(input.value, 10);
			if (v === secret) nextStep();
			else
				hint.textContent =
					v < secret ? "C'est plus grand ↑" : "C'est plus petit ↓";
			console.log(`Devinez le nombre: ${secret}`);
		});
	});

	steps.push(() => {
		root.innerHTML = `
      <div class="captcha-card">
        <h1>Mini casse-brique</h1>
        <canvas id="brickGame" width="400" height="300" style="background:#eee;"></canvas>
      </div>
    `;
		const canvas = document.getElementById("brickGame");
		const c = canvas.getContext("2d");

		const paddle = { x: 150, y: 260, w: 100, h: 10 };
		const ball = { x: 200, y: 150, dx: 3, dy: 3, r: 8 };
		const bricks = Array.from({ length: 6 }, (_, i) => ({
			x: 20 + i * 60,
			y: 40,
			w: 50,
			h: 20,
			active: true,
		}));

		document.addEventListener("mousemove", (e) => {
			paddle.x = e.clientX - canvas.getBoundingClientRect().left - paddle.w / 2;
		});

		function loop() {
			c.clearRect(0, 0, 400, 300);
			c.fillStyle = "#111";
			c.fillRect(paddle.x, paddle.y, paddle.w, paddle.h);

			c.beginPath();
			c.arc(ball.x, ball.y, ball.r, 0, Math.PI * 2);
			c.fill();

			bricks.forEach((b) => {
				if (!b.active) return;
				c.fillStyle = "#e63946";
				c.fillRect(b.x, b.y, b.w, b.h);
				if (
					ball.x > b.x &&
					ball.x < b.x + b.w &&
					ball.y > b.y &&
					ball.y < b.y + b.h
				) {
					b.active = false;
					ball.dy *= -1;
				}
			});

			ball.x += ball.dx;
			ball.y += ball.dy;

			if (ball.x < ball.r || ball.x > 400 - ball.r) ball.dx *= -1;
			if (ball.y < ball.r) ball.dy *= -1;

			if (
				ball.y + ball.r >= paddle.y &&
				ball.y - ball.r <= paddle.y + paddle.h &&
				ball.x + ball.r >= paddle.x &&
				ball.x - ball.r <= paddle.x + paddle.w
			) {
				ball.dy *= -1;
				ball.y = paddle.y - ball.r;
			}

			if (ball.y > 300 - ball.r) {
				ball.x = 200;
				ball.y = 150;
				ball.dx = 3;
				ball.dy = 3;
			}

			if (bricks.every((b) => !b.active)) {
				nextStep();
				return;
			}

			requestAnimationFrame(loop);
		}
		loop();
	});

	// Autres steps (différences, QCM, ne bougez plus, labyrinthe)...

	steps.push(() => {
		root.innerHTML = `
      <div class="captcha-card">
        <h1>🎉 Félicitations !</h1>
        <p style="font-size:20px;margin-top:20px;">Vous êtes désormais inscrit sur notre site.</p>
        <p style="margin-top:15px;font-size:16px;color:#666;">Merci d'avoir complété toutes les étapes de vérification.</p>
        <button id="finishBtn" style="margin-top:30px;padding:14px 28px;font-size:18px;">Terminer</button>
      </div>
    `;

		document.getElementById("finishBtn").addEventListener("click", () => {
			root.innerHTML = `
        <h1>✅ Inscription réussie !</h1>
        <p style="margin-top:20px;font-size:18px;color:#444;">Bienvenue parmi nous.</p>
      `;
		});
	});

	nextStep();
})();
