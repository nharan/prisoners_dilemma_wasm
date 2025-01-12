import init, { run_simulation } from '../pkg/prisoners_dilemma.js';

    async function main() {
        try {
            await init();

            const runButton = document.getElementById('runButton');
            const leaderboardDiv = document.getElementById('leaderboard');

            runButton.addEventListener('click', async () => {
                try {
                    const leaderboardJson = run_simulation();
                    console.log("Leaderboard JSON:", leaderboardJson);
                    const leaderboard = JSON.parse(leaderboardJson);
                    displayLeaderboard(leaderboard);
                } catch (error) {
                     console.error("Error running simulation or parsing JSON:", error);
                    leaderboardDiv.innerHTML = `<p>Error: ${error.message}</p>`;
                }
            });

            function displayLeaderboard(leaderboard) {
                let html = '<h2>Leaderboard</h2><ol>';
                leaderboard.forEach(item => {
                    html += `<li>${item.strategy}: ${item.score}</li>`;
                });
                html += '</ol>';
                leaderboardDiv.innerHTML = html;
            }
        } catch (error) {
            console.error("Error initializing WASM:", error);
            document.getElementById('leaderboard').innerHTML = `<p>Error: ${error.message}</p>`;
        }
    }

    main();
