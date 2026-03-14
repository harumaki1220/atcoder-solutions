#include <bits/stdc++.h>
using namespace std;

int main()
{
    int N;
    cin >> N;

    vector<string> S(N);
    for (int i = 0; i < N; i++)
    {
        cin >> S.at(i);
    }

    int length = 0;
    for (int i = 0; i < N; i++)
    {
        if (S.at(i).size() > length)
        {
            length = S.at(i).size();
        }
    }

    int space = 0;
    for (int i = 0; i < N; i++)
    {
        if (S.at(i).size() < length)
        {
            space = (length - S.at(i).size()) / 2;
            for (int j = 0; j < space; j++)
            {
                cout << ".";
            }
            cout << S.at(i);
            for (int j = 0; j < space; j++)
            {
                cout << ".";
            }
            cout << endl;
        }
        else
        {
            cout << S.at(i) << endl;
        }
    }
}
