#include <stdio.h>

#define MAX 55

int main() {
	int x, y, i, j, h, w;
	char f[MAX][MAX];
	scanf("%d %d", &h, &w);
	for(y=1; y<=h; y++)
	    scanf("%s", f[y]+1);
	for(y=1; y<=h; y++)
	    for(x=1; x<=w; x++)
	        if(f[y][x]=='.'){
		        f[y][x] = '0';
		        for(i=-1; i<=1; i++)
		            for(j=-1; j<=1; j++)
		                if(f[y+i][x+j]=='#')
			                f[y][x]++;
	        }
	for(y=1; y<=h; y++)
	    printf("%s\n", f[y]+1);
	
	return 0;
}
