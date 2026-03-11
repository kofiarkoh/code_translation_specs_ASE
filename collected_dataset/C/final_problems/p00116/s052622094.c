#include <strings.h>
#include <stdio.h>

int min(int a, int b)
{
    return (a > b ? b : a);
}
 
int max(int a, int b)
{
    return (b > a ? b : a);
}
 
int main(void) {
    int i, j, k, h, w;
    int m[501][501];
    int area, hmin;
    char map[500][501];
    int cnt[500][500];

    while (scanf("%d %d", &h, &w), h + w) {
	area = 0;
#if 1
	getchar();
	bzero(m, sizeof(m));
	for (i = 0; i < h; i++) {
	    for (j = 0; j < w; j++) {
		char c;
		scanf("%c", &c);
		if (c == '*') c = 0;
		else {
		    if (i == 0) c = 1;
		    else c = m[i-1][j]+1;
		}
		m[i][j] = c;
	    }
	    getchar();
	}
#else
        for (i = 0; i < h; i++){
            scanf("%s", map[i]);
        }
         
        for (i = 0; i < h; i++){
            for (j = 0; j < w; j++){
                if (i == 0){
                    cnt[i][j] = map[i][j] == '*' ? 0 : 1;
                }
                else {
                    if (map[i][j] == '.'){
                        cnt[i][j] = cnt[i - 1][j] + 1;
                    }
                    else {
                        cnt[i][j] = 0;
                    }
                }
            }
        }
#endif
/*
	for (i = 0; i < h; i++) {
	    for (j = 0; j < w; j++) {
		printf("%d", m[i][j]);
	    }
	    putchar('\n');
	}
*/
#if 1
	for (i = 0; i < h; i++) {
	    for (j = 0; j < w; j++) {
		hmin = 10000000;
		for (k = j; k < w; k++) {
		    if (m[i][k] < hmin) hmin = m[i][k];
		    if (area < hmin * (k-j+1)) area = hmin * (k-j+1);
		}
	    }
	}
#else
        for (i = 0; i < h; i++){
            for (j = 0; j < w; j++){
                hmin = 10000000;
                for (k = j; k < w; k++){
                    hmin = min(cnt[i][k], hmin);
                    area = max(area, (k - j + 1) * hmin);
                }
            }
        }
#endif
	printf("%d\n", area);
    }
    return 0;
}