(function () {
    const params = new URLSearchParams(window.location.search);
    const status = params.get("waitlist");
    if (!status) return;

    params.delete("waitlist");
    const qs = params.toString();
    history.replaceState({}, "", window.location.pathname + (qs ? "?" + qs : ""));

    if (status === "joined") {
        const card = document.querySelector(".waitlist-card");
        if (card) {
            const title = card.querySelector(".waitlist-card-title");
            if (title) title.textContent = "🎉 You're on the waitlist!";
            const row = card.querySelector(".waitlist-cta-row");
            if (row) {
                row.innerHTML =
                    '<p class="waitlist-cta-note">Thanks for signing up - we\'ll email you when the dashboard launches.</p>';
            }
        }
        return;
    }

    const toast = document.createElement("div");
    toast.className = "waitlist-toast";
    toast.textContent = "Something went wrong - please try again.";
    document.body.appendChild(toast);
    setTimeout(() => toast.remove(), 6000);
})();
