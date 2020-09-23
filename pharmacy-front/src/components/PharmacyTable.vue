<template>
  <div class="pharmacy-list">
    <div class="pharmacy-list-searchbox">
      <label for="searchbox" class="pharmacy-list-searchbox-label">
        Chercher une pharmacie par raison sociale
      </label>
      <input
          type="search"
          class="pharmacy-list-searchbox-input"
          id="searchbox"
          :placeholder="`ex : ${pharmacies[0].rs}`"
          @keyup="setFilter"
      >
    </div>
    <table class="pharmacy-list-table">
      <thead>
      <tr>
        <th>Raison sociale</th>
        <th>Adresse postale</th>
        <th>Téléphone</th>
        <th>Télécopie</th>
      </tr>
      </thead>
      <tbody>
      <tr v-for="pharmacy of filteredPharmacies" :key="pharmacy.rs">
        <td>{{pharmacy.rs}}</td>
        <td>{{pharmacy.numvoie}} {{pharmacy.compvoie}} {{pharmacy.typvoie}} {{pharmacy.voie}} {{pharmacy.cp}} {{pharmacy.commune}}</td>
        <td>(+33) {{pharmacy.telephone}}</td>
        <td>(+33) {{pharmacy.telecopie}}</td>
      </tr>
      </tbody>
    </table>
  </div>
</template>

<script>
export default {
  name: "PharmacyTable",
  props: {
    pharmacies: Array
  },
  data() {
    return {
      filter: ""
    }
  },
  computed: {
    filteredPharmacies() {
      return this.pharmacies.filter(pharmacy => {
        return pharmacy.rs.toUpperCase().includes(this.filter.toUpperCase());
      })
    }
  },
  methods: {
    setFilter(e) {
      this.filter = e.target.value;
    }
  }
}
</script>

<style scoped lang="scss">
.pharmacy-list {
  &-searchbox {
    margin-bottom: 24px;
    display: flex;
    flex-direction: column;
    &-input {
      padding: 8px;
      margin-top: 8px;
      border: 1px solid #b7b7b7;
      border-radius: 4px;
    }
  }
  &-table {
    border-collapse: collapse;
    box-sizing: border-box;
    width: 100%;
    tr td, tr th {
      border: 1px solid #b7b7b7;
      padding: 8px;
    }
  }
}
</style>
