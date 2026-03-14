#include <bits/stdc++.h>
using namespace std;

int main()
{
    int N, K;
    cin >> N >> K;

    int total = N, years = 0;
    while (total < K)
    {
        years++;
        N++;
        total += N;
    }

    cout << years << endl;
}