#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int addmod(int a, int b, int m)
{
    return ((a % m) + (b % m)) % m;
}

int mulmod(int a, int b, int m)
{
    int result = a;
    for (int i = 1; i < b; i++) {
        result = addmod(result, a, m);
    }
    return result;
}

int powmod(int x, int n, int m)
{
    int result = 1;
    while (n--)
    {
        result = mulmod(result, x, m);
    }
    return result;
}

int main()
{
    char str[100007];
    int wNum = 0;
    int aCount = 0, aCountNew = 0, abCount = 0, abCountNew = 0, abcCount = 0, abcCountNew = 0, len, modNum = 1e9 + 7;
    gets(str);
    if (str[0] == 'A')
    {
        aCountNew = ++aCount;
    }
    else if (str[0] == '?')
    {
        aCountNew = ++aCount;
        wNum++;
    }
    len = strlen(str);
    for (int i = 1; i < len; i++) {
        if (str[i] == 'A') {
            aCountNew = addmod(aCount, powmod(3, wNum, modNum), modNum);
        }
        else if (str[i] == 'B') {
            abCountNew = addmod(aCount, abCount, modNum);
        }
        else if (str[i] == 'C') {
            abcCountNew = addmod(abCount, abcCount, modNum);
        }
        else {
            aCountNew = addmod(mulmod(aCount, 3, modNum), powmod(3, wNum, modNum), modNum);
            abCountNew = addmod(mulmod(abCount, 3, modNum), aCount, modNum);
            abcCountNew = addmod(mulmod(abcCount, 3, modNum), abCount, modNum);
            wNum++;
        }
        aCount = aCountNew;
        abCount = abCountNew;
        abcCount = abcCountNew;
    }
    printf("%d", abcCount);
    return 0;
}
