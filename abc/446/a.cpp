#include <bits/stdc++.h>
using namespace std;

int main()
{
    string S;
    cin >> S;

    transform(S.begin(), S.end(), S.begin(), ::tolower);

    cout << "Of";
    cout << S << endl;
}
