export default function AddressPage({ params }: { params: { id: string } }) {
  return (
    <main className="min-h-screen bg-gradient-to-br from-slate-900 to-slate-800">
      <div className="container mx-auto px-4 py-16">
        <h1 className="text-3xl font-bold text-white mb-4">Address</h1>
        <p className="text-slate-300 mb-8">Address: {params.id}</p>

        {/* TODO: Fetch address data from API */}
        {/* TODO: Display address balance */}
        {/* TODO: Display transaction history */}

        <div className="bg-slate-700 p-6 rounded-lg">
          <p className="text-slate-300">Address details loading...</p>
        </div>
      </div>
    </main>
  );
}
