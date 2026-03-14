#include <bits/stdc++.h>
using namespace std;

int main()
{
    int N;
    cin >> N;

    vector<int> A(N);
    for (int i = 0; i < N; i++)
    {
        cin >> A.at(i);
    }

    long long sum = 0;
    for (int i = 0; i < N; i++)
        sum += A.at(i);

    sort(A.begin(), A.end());

    long long L1 = A[N - 1];
    long long L2 = (long long)A[0] + A[N - 1];

    bool first = true;

    if (sum % L1 == 0)
    {
        cout << L1;
        first = false;
    }

    if (sum % L2 == 0 && L1 != L2)
    {
        if (!first)
            cout << " ";
        cout << L2;
    }

    cout << endl;
}
