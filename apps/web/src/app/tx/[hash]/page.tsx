export default function TransactionPage({
  params,
}: {
  params: { hash: string };
}) {
  return (
    <main className="min-h-screen bg-gradient-to-br from-slate-900 to-slate-800">
      <div className="container mx-auto px-4 py-16">
        <h1 className="text-3xl font-bold text-white mb-4">Transaction</h1>
        <p className="text-slate-300 mb-8">Hash: {params.hash}</p>

        {/* TODO: Fetch transaction data from API */}
        {/* TODO: Display transaction details */}
        {/* TODO: Display related events */}

        <div className="bg-slate-700 p-6 rounded-lg">
          <p className="text-slate-300">Transaction details loading...</p>
        </div>
      </div>
    </main>
  );
}
