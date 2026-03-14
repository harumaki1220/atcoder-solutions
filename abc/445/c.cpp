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

    vector<int> ans(N + 1);

    for (int i = N; i >= 1; i--)
    {
        int current_val = A.at(i - 1);

        if (current_val == i)
        {
            ans[i] = i;
        }
        else
        {
            ans[i] = ans[current_val];
        }
    }

    for (int i = 1; i <= N; i++)
    {
        if (i == N)
        {
            cout << ans.at(i) << endl;
        }
        else
        {
            cout << ans.at(i) << " ";
        }
    }
}
