#include<stdio.h>
#define ulli unsigned long long int
int min(int a, int b)
{
    if (a < b)
        return a;
    else
        return b;
}
ulli min_ll(ulli a, ulli b)
{
    if (a < b)
        return a;
    else
        return b;
}
ulli abs(long long int a)
{
    if (a < 0)
        return -a;
    else
        return a;
}
int main()
{
    int a, b, q;
    scanf("%d %d %d", &a, &b, &q);
    int i;
    ulli s[100005], t[100005], x[100005];
    for (i = 0; i < a; i++)
        scanf("%lld", &s[i]);
    for (i = 0; i < b; i++)
        scanf("%lld", &t[i]);
    for (i = 0; i < q; i++)
        scanf("%lld", &x[i]);
    ulli f[10005], g[10005];
    int j;
    j = 0;
    for (i = 0; i < a; i++)
    {
        while (abs(s[i] - t[j]) > abs(s[i] - t[min(j + 1, b - 1)]))
        {
            j++;
            if (j == b)
                j--;
        }
        f[i] = abs(s[i] - t[j]);
    }
    j = 0;
    for (i = 0; i < b; i++)
    {
        while (abs(t[i] - s[j]) > abs(t[i] - s[min(j + 1, a - 1)]))
        {
            j++;
            if (j == a)
                j--;
        }
        g[i] = abs(t[i] - s[j]);
    }
    ulli  mini, max, mid;
    ulli ans;
    for (i = 0; i < q; i++)
    {
        if (x[i] <= s[0])
        {
            ans = s[0] - x[i] + f[0];
        }
        else if (x[i] >= s[a - 1])
        {
            ans = x[i] - s[a - 1] + f[a - 1];
        }
        else
        {
            mini = 0;
            max = a - 1;
            mid = (max + mini) / 2;
            while (max - mini > 1)
            {
                if (s[mid] < x[i])
                    mini = mid;
                else if (s[mid] > x[i])
                    max = mid;
                else
                    break;
                mid = (max + mini) / 2;
            }
            if (s[mid] == x[i])
                ans = f[mid];
            else
                ans = min_ll(x[i] - s[mini] + f[mini], s[max] - x[i] + f[max]);
        }
        if (x[i] <= t[0])
        {
            ans = min_ll(t[0] - x[i] + g[0], ans);
        }
        else if (x[i] >= t[b - 1])
        {
            ans = min_ll(x[i] - t[b - 1] + g[b - 1], ans);
        }
        else
        {
            mini = 0;
            max = b - 1;
            mid = (max + mini) / 2;
            while (max - mini > 1)
            {
                if (t[mid] < x[i])
                    mini = mid;
                else if (t[mid] > x[i])
                    max = mid;
                else
                    break;
                mid = (max + mini) / 2;
            }
            if (t[mid] == x[i])
                ans = g[mid];
            else
            {
                ans = min_ll(x[i] - t[mini] + g[mini], ans);
                ans = min_ll(t[max] - x[i] + g[max], ans);
            }
        }
        printf("%lld\n", ans);
    }
    return 0;
}