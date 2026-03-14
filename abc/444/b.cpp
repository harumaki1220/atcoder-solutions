#include <bits/stdc++.h>
using namespace std;

int sumOfDigits(int n)
{
    int sum = 0;
    while (n > 0)
    {
        sum += n % 10;
        n /= 10;
    }
    return sum;
}

int main()
{
    int N, K;
    cin >> N >> K;

    int count = 0;

    for (int i = 1; i <= N; i++)
    {
        int digitSum = sumOfDigits(i);

        if (digitSum == K)
        {
            count++;
        }
    }

    cout << count << endl;
}
