window.addEventListener('scroll', function() {
    const nav = document.querySelector('nav');
    if (window.scrollY > 0) {
        nav.classList.add('scrolled');  // Añade la clase cuando se hace scroll
    } else {
        nav.classList.remove('scrolled');  // Elimina la clase cuando no hay scroll
    }
});

