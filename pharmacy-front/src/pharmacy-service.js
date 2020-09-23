
export async function get_pharmacies(page) {
    let req = await fetch(`http://localhost:8000/?page=${page}`);
    return await req.json()
}
