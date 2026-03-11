#include <stdio.h>
typedef long long int ll;

ll factorial(ll n, ll md)
{
    ll res = 1;
    for (int i = 1; i < n+1; ++i)
    {
        res *= i;
        res %= md;
    }
    return res;
}

int main()
{
    ll N, ans;
    scanf("%lld", &N);
    char S[200005];
    scanf("%s", S);

    if (S[0] == 'W' || S[-1] == 'W')
    {
        puts("0");
        return 0;
    }
    else ans = 1;

    int op[200005] = {};
    ll ct_L = 1;
    ll ct = 1;
    for (int i = 1; i < 2*N; ++i)
    {
        if (S[i] == S[i-1])
        {
            if (op[i-1] == 0)
            {
                op[i] = 1;
                ans *= ct_L;
                ct_L--;
            }
            else
            {
                op[i] = 0;
                ct_L++;
                ct++;
            }
        }
        else
        {
            if (op[i-1] == 0)
            {
                op[i] = 0;
                ct_L++;
                ct++;
            }
            else
            {
                op[i] = 1;
                ans *= ct_L;
                ct_L--;
            }
        }
    }



    if (ct != N)
    {
        puts("0");
        return 0;
    }

    ll mod = 1000000007;

    printf("%lld\n", factorial(N, mod) * ans % mod);

    return 0;
}
