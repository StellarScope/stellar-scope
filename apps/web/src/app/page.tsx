export default function Home() {
  return (
    <main className="min-h-screen bg-gradient-to-br from-slate-900 to-slate-800">
      <div className="container mx-auto px-4 py-16">
        <h1 className="text-4xl font-bold text-white mb-4">StellarScope</h1>
        <p className="text-xl text-slate-300 mb-8">
          Blockchain explorer for Stellar network
        </p>

        {/* TODO: Add search bar */}
        {/* TODO: Add recent transactions */}
        {/* TODO: Add network stats */}

        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mt-12">
          <div className="bg-slate-700 p-6 rounded-lg">
            <h2 className="text-lg font-semibold text-white mb-2">
              Transactions
            </h2>
            <p className="text-slate-300">TODO: Display transaction count</p>
          </div>
          <div className="bg-slate-700 p-6 rounded-lg">
            <h2 className="text-lg font-semibold text-white mb-2">Addresses</h2>
            <p className="text-slate-300">TODO: Display address count</p>
          </div>
          <div className="bg-slate-700 p-6 rounded-lg">
            <h2 className="text-lg font-semibold text-white mb-2">Events</h2>
            <p className="text-slate-300">TODO: Display event count</p>
          </div>
        </div>
      </div>
    </main>
  );
}
