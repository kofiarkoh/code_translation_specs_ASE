#include <stdio.h>
#include <string.h>
#include <math.h>
int main()
{
    int n;
    char s[11];
    long long int ans = 0;
    int a = 0, b = 0, ab = 0;
    int len;
    scanf("%d", &n);
    for (int i = 0; i < n; i++)
    {
        scanf("%s", s);
        len = strlen(s);
        if (s[len - 1] == 'A' && s[0] == 'B')
        {
            ab++;
        }
        else if (s[0] == 'B')
        {
            b++;
        }
        else if (s[len - 1] == 'A')
        {
            a++;
        }
        for (int i = 0; i < len - 1; i++)
        {
            if (s[i] == 'A' && s[i + 1] == 'B')
            {
                ans++;
            }
        }
    }
    if (a == 0 && b == 0)
    {
        ans += fmaxl(fminl(ab - 1, n - 1), 0);
    }
    else
    {
        ans += fmaxl(fminl(fminl(a + ab, b + ab), n - 1), 0);
    }
    printf("%lld", ans);
    return 0;
}