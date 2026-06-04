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
            card.innerHTML =
                '<h2 class="waitlist-card-title">🎉 You\'re on the waitlist!</h2>' +
                '<p>Thanks for signing up - we\'ll email you when the dashboard launches.</p>';
        }
        return;
    }

    const toast = document.createElement("div");
    toast.className = "waitlist-toast";
    toast.textContent = "Something went wrong - please try again.";
    document.body.appendChild(toast);
    setTimeout(() => toast.remove(), 6000);
})();
