<template>
    <!-- Make the navbar with routes -->
    <nav>
        <RouterLink class="route-button" to="/Upload"><svg style="cursor: pointer;" xmlns="http://www.w3.org/2000/svg"
                viewBox="0 -960 960 960" fill="#00000">
                <path
                    d="M220-160q-24 0-42-18t-18-42v-113q0-12.75 8.68-21.38 8.67-8.62 21.5-8.62 12.82 0 21.32 8.62 8.5 8.63 8.5 21.38v113h520v-113q0-12.75 8.68-21.38 8.67-8.62 21.5-8.62 12.82 0 21.32 8.62 8.5 8.63 8.5 21.38v113q0 24-18 42t-42 18H220Zm230-524-99 99q-8.8 9-20.9 8.5-12.1-.5-21.49-9.5-8.61-9-8.61-21.5t9-21.5l150-150q5-5 10.13-7 5.14-2 11-2 5.87 0 10.87 2 5 2 10 7l151 151q9 9 9 21t-8.61 21q-9.39 9-21.89 9t-21.5-9l-99-98v341q0 12.75-8.68 21.37-8.67 8.63-21.5 8.63-12.82 0-21.32-8.63-8.5-8.62-8.5-21.37v-341Z" />
            </svg>
            <p class="show-tooltip">Upload notes</p>
        </RouterLink>
        <RouterLink class="route-button" to="/Download"><svg style="cursor: pointer;" xmlns="http://www.w3.org/2000/svg"
                viewBox="0 -960 960 960" fill="#00000">
                <path
                    d="M479.87-325q-5.87 0-10.87-2-5-2-10-7L308-485q-9-9.27-8.5-21.64.5-12.36 9.11-21.36 9.39-9 21.89-9t21.5 9l98 99v-341q0-12.75 8.68-21.38 8.67-8.62 21.5-8.62 12.82 0 21.32 8.62 8.5 8.63 8.5 21.38v341l99-99q8.8-9 20.9-8.5 12.1.5 21.49 9.5 8.61 9 8.61 21.5t-9 21.5L501-334q-5 5-10.13 7-5.14 2-11 2ZM220-160q-24 0-42-18t-18-42v-113q0-12.75 8.68-21.38 8.67-8.62 21.5-8.62 12.82 0 21.32 8.62 8.5 8.63 8.5 21.38v113h520v-113q0-12.75 8.68-21.38 8.67-8.62 21.5-8.62 12.82 0 21.32 8.62 8.5 8.63 8.5 21.38v113q0 24-18 42t-42 18H220Z" />
            </svg>
            <p class="show-tooltip">Download notes</p>
        </RouterLink>
        <RouterLink class="route-button" to="#"><svg style="cursor: pointer;" xmlns="http://www.w3.org/2000/svg"
                viewBox="0 -960 960 960" fill="#00000">
                <path
                    d="M480.09-336.92q67.99 0 115.49-47.59t47.5-115.58q0-67.99-47.59-115.49t-115.58-47.5q-67.99 0-115.49 47.59t-47.5 115.58q0 67.99 47.59 115.49t115.58 47.5ZM480-392q-45 0-76.5-31.5T372-500q0-45 31.5-76.5T480-608q45 0 76.5 31.5T588-500q0 45-31.5 76.5T480-392Zm0 172q-126.31 0-231.04-67.39-104.73-67.38-167.19-177.3-5-8.62-7.31-17.37-2.3-8.75-2.3-17.96t2.3-17.94q2.31-8.73 7.31-17.35 62.46-109.92 167.19-177.3Q353.69-780 480-780q126.31 0 231.04 67.39 104.73 67.38 167.19 177.3 5 8.62 7.31 17.37 2.3 8.75 2.3 17.96t-2.3 17.94q-2.31 8.73-7.31 17.35-62.46 109.92-167.19 177.3Q606.31-220 480-220Zm0-280Zm0 220q113 0 207.5-59.5T832-500q-50-101-144.5-160.5T480-720q-113 0-207.5 59.5T128-500q50 101 144.5 160.5T480-280Z" />
            </svg>
            <p class="show-tooltip">View notes</p>
        </RouterLink>
        <RouterLink class="route-button" to="#"><svg style="cursor: pointer;" xmlns="http://www.w3.org/2000/svg"
                viewBox="0 -960 960 960" fill="#00000">
                <path
                    d="M292.31-140q-29.83 0-51.07-21.24Q220-182.48 220-212.31V-720h-10q-12.75 0-21.37-8.63-8.63-8.63-8.63-21.38 0-12.76 8.63-21.37Q197.25-780 210-780h150q0-14.69 10.35-25.04 10.34-10.34 25.03-10.34h169.24q14.69 0 25.03 10.34Q600-794.69 600-780h150q12.75 0 21.37 8.63 8.63 8.63 8.63 21.38 0 12.76-8.63 21.37Q762.75-720 750-720h-10v507.69q0 29.83-21.24 51.07Q697.52-140 667.69-140H292.31ZM680-720H280v507.69q0 5.39 3.46 8.85t8.85 3.46h375.38q5.39 0 8.85-3.46t3.46-8.85V-720ZM406.17-280q12.75 0 21.37-8.62 8.61-8.63 8.61-21.38v-300q0-12.75-8.63-21.38-8.62-8.62-21.38-8.62-12.75 0-21.37 8.62-8.61 8.63-8.61 21.38v300q0 12.75 8.62 21.38 8.63 8.62 21.39 8.62Zm147.69 0q12.75 0 21.37-8.62 8.61-8.63 8.61-21.38v-300q0-12.75-8.62-21.38-8.63-8.62-21.39-8.62-12.75 0-21.37 8.62-8.61 8.63-8.61 21.38v300q0 12.75 8.63 21.38 8.62 8.62 21.38 8.62ZM280-720v520-520Z" />
            </svg>
            <p class="show-tooltip">Remove notes</p>
        </RouterLink>
    </nav>
</template>
<script setup>
import { RouterLink, RouterView } from 'vue-router'


</script>
<style>
nav {
    position: fixed;
    top: 0;
    left: 0;
    width: 4%;
    height: 100%;
    margin-left: 0.5%;
}

nav a {
    position: relative;
    display: flex;
    align-items: center;
    cursor: default;
}

nav :first-child {
    margin-top: 117%;
}

nav a svg {
    width: 100%;
}

nav a p {
    transition: transform 0.2s cubic-bezier(0.68, -0.55, 0.27, 1.55), opacity 0.2s ease-in-out;
    font-family: "Poppins", sans-serif;
    background-color: black;
    color: #fff;
    text-align: center;
    padding: 5px 10px;
    border-radius: 6px;
    position: absolute;
    left: 130%;
    top: 70%;
    transform: translateY(-50%);
    z-index: 999;
    white-space: nowrap;
    opacity: 0;
    transform-origin: center center;
    transform: translateY(-70%) scale(0.5);
}

nav a svg:hover+p {
    opacity: 1;
    transform: translateY(-70%) scale(0.9);
}
</style>
