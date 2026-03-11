#include <stdio.h>
char a[16];
int main(void)
{
    while (scanf("%s", a) != EOF) {
        int i, j;
        char res=0;
        for (i=0; i<3; ++i) {
            if (a[3*i] != 's' && a[3*i] == a[3*i+1] && a[3*i] == a[3*i+2]) {
                res=a[3*i];
                break;
            }
        }
        if (!res) for (i=0; i<3; ++i) {
            if (a[3*i] != 's' && a[3*i] == a[3*i+3] && a[3*i] == a[3*i+6]) {
                res=a[3*i];
                break;
            }
        }
        if (!res) {
            if (a[0] != 's' && a[0] == a[4] && a[0] == a[8]) {
                res=a[0];
            }
            else if (a[2] != 's' && a[2] == a[4] && a[2] == a[6]) {
                res=a[3];
            }
        }
        if (!res) res='d';
        printf("%c\n", res);
    }
    return 0;
}