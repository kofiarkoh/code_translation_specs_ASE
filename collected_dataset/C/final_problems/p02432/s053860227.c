#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define BASE 400000

char Q[800001][12];
int top = BASE;
int tail = BASE;
char a[30];

int main() {
    int q;
    fgets(a, 30, stdin);
    q = atoi(a);
    while (q--) {
        fgets(a, 30, stdin);
        if (a[0] == '0') {
            if (a[2] == '0') strcpy(Q[--top], a+4);
            else             strcpy(Q[tail++], a+4);
        } else if (a[0] == '1') {
            printf(Q[top+atoi(a+2)]);
        } else {
            if (a[2] == '0') top++;
            else tail--;
        }
    }
}
