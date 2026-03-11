#include <stdio.h>
#include <string.h>
#define MAX 21
 
void init(int);
void move(int);
 
int n, x, y;
char fld[MAX][MAX];
 
int main(void) {
    int m;
    while(~scanf("%d", &n), n) {
        init(n);
        scanf("%d", &m);
        move(m);
        printf("%s\n", n ? "No" : "Yes");
    }
    return 0;
}
 
void init(int i) {
    int a, b;
    x = y = 10;
    memset(fld, 0, sizeof(fld));
    while(i--) {
        scanf("%d %d", &a, &b);
        fld[b][a]++;
    }
}
 
void move(int i) {
    int a, d;
    char b[2];
    while(i--) {
        scanf("%1s %d", b, &a);
        while(a--) {
            *(*b % 75 > 10 ? &x : &y) += *b < 80 ? 1 : -1;
            fld[y][x] && (fld[y][x]--, n--);
        }
    }
}