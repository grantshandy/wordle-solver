import init, {wordle} from './wordle.js';

const app = new Vue({
    el: '#app',

    data: {
        known_1: '',
        known_2: '',
        known_3: '',
        known_4: '',
        known_5: '',
        not_in_word: '',
        in_word: '',
        results: [],
    },

    async created() {
        await init();

        console.log('Vue Initialized');
    },

    methods: {
        getResults: function() {
            this.results = wordle([this.known_1, this.known_2, this.known_3, this.known_4, this.known_5], this.not_in_word, this.in_word);
            console.log(this.results);
        }
    }
});