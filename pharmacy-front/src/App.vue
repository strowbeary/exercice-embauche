<template>
  <div id="app">
    <PharmacyTable :pharmacies="pharmacies"/>
    <div class="pagination">
      <button @click="prevPage">Previous</button>
      <span class="pagination-indicator">{{ page + 1 }} / {{ Math.floor(totalNbs / 10) + 1 }}</span>
      <button @click="nextPage">Next</button>
    </div>
  </div>
</template>

<script>
import {get_pharmacies} from "@/pharmacy-service";
import PharmacyTable from "@/components/PharmacyTable";

export default {
  name: 'App',
  components: {
    PharmacyTable
  },
  data () {
    return {
      page: 0,
      totalNbs: 0,
      pharmacies: []
    }
  },
  beforeMount () {
    this.loadPharmacies();
  },
  methods: {
    prevPage () {
      this.page -= 1;
      this.loadPharmacies();
    },
    nextPage () {
      if(this.page < Math.floor(this.totalNbs / 10)) {
        this.page += 1;
        this.loadPharmacies();
      }
    },
    loadPharmacies () {
      get_pharmacies(this.page)
          .then(paginated_results => {
            this.totalNbs = paginated_results.total_nbs_results;
            this.pharmacies = paginated_results.pharmacies;
            this.page = paginated_results.page;
          });
    }
  }
}
</script>

<style lang="scss">
body {
  margin: 20px;
}

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
  width: 100%;
}
.pagination {
  margin-top: 24px;
  display: flex;
  justify-content: center;
  &-indicator {
    margin: 0 8px;
  }
}
</style>
