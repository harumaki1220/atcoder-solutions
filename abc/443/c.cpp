#include <bits/stdc++.h>
using namespace std;

int main()
{
    int N, T;
    cin >> N >> T;

    vector<int> A(N);
    for (int i = 0; i < N; i++)
    {
        cin >> A.at(i);
    }

    int total = 0;
    int open_at = 0;
    for (int i = 0; i < N; i++)
    {
        if (A.at(i) > open_at)
        {
            total += A.at(i) - open_at;
            open_at = A.at(i) + 100;
        }
    }

    // もし最終時刻 T までに再び開くなら、その分も足す
    if (T > open_at)
    {
        total += T - open_at;
    }

    cout << total << endl;
}
