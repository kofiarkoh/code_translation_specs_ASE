#include<stdio.h>
#include<string.h>
int main()
{
    char s[30];
    scanf("%s", s);
    int l = strlen(s);
    int i;
    int x = 0;
    if (l == 26)
    {
        for (i = l - 2; i >= 0; i--)
        {
            if (s[i] < s[i + 1])
            {
                x++;
                break;
            }
        }
        if (x == 0)
        {
            printf("-1\n");
            return 0;
        }
        else if (s[l - 1] != 'a')
        {
            int a;
            char last = s[l - 1];
            for (i = l - 2; i >= 0; i--)
            {
                if (s[i] - last < 0)
                {
                    a = i;
                    break;
                }
            }
            for (i = 0; i < a; i++)
            {
                printf("%c", s[i]);
            }
            printf("%c\n", last);
            return 0;
        }
        else
        {
            char c;
            int a;
            for (i = l - 1; i >= 0; i--)
            {
                if (s[i - 1] - s[i] != 1)
                {
                    c = s[i - 1];
                    break;
                }
            }
            i -= 2;
            for (; i >= 0; i--)
            {
                if (s[i] - c < 0)
                {
                    a = i;
                    break;
                }
            }
            for (i = 0; i < a; i++)
                printf("%c", s[i]);
            printf("%c\n", c);
            return 0;
        }
    }
    else
    {
        int j;
        for (j = 0; j < 26; j++)
        {
            x = 0;
            for (i = 0; i < l; i++)
            {
                if (s[i] == 'a' + j)
                {
                    x++;
                    break;
                }
            }
            if (x == 0)
            {
                printf("%s%c\n", s, 'a' + j);
                return 0;
            }
        }
    }
}