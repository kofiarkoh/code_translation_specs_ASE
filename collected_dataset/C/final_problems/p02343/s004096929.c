#include <stdio.h>

int main(void)
{
    int n, q;
    int i, j;
    int com[100000], x[100000], y[100000];
    int group[100000];
    int tmp;

    scanf("%d %d", &n, &q);
    for (i = 0; i < q; i++)
    {
        scanf("%d %d %d", &com[i], &x[i], &y[i]);
        group[i] = i;
        if (com[i] == -1)
        {
            break;
        }
    }

    for (i = 0; i < q; i++)
    {
        if (com[i] == 0) //unite
        {
            tmp = group[y[i]];
            for (j = 0; j < q; j++)
            {
                if (group[j] == tmp)
                {
                    group[j] = group[x[i]];
                }
            }
        }
        if (group[x[i]] == group[y[i]] && com[i] == 1)
        {
            printf("1\n");
        }
        else if (com[i] == 1)
        {
            printf("0\n");
        }
    }
    return 0;
}
