#include <stdio.h>
#include <string.h>

int main(void)
{
    char ss[100];
    int l, i, j, ans_l, flg = 1;

    scanf("%s", ss);
    l = strlen(ss);

    ans_l = l - 7;


    for (i = 0; i < l; i++) {
        char ans[8];
        for (j = 0; j < l; j++) {
            if (j < i)
                ans[j] = ss[j];
            else if(j > i + ans_l - 1)
                ans[j - ans_l] = ss[j];
            }
        ans[7] = '\0';


        if (strcmp(ans, "keyence") == 0) {
            flg = 0; break;
        }
    }

    if (flg == 0)
        printf("YES");
    else
        printf("NO");

    return 0;
}
