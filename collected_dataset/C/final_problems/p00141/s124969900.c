#include <stdio.h>

int dx[] = { 0, 1, 0,-1};
int dy[] = {-1, 0, 1, 0};

int main(void){
	int i,j,SET;
	int T,n;
	int x,y,d,nx,ny,nnx,nny;
	int cnt,firstFlg;
	char s[102][102];

	scanf("%d",&T);

	for(SET=1;SET<=T;SET++){
		if(SET != 1) putchar('\n');

		scanf("%d",&n);
		for(i=0;i<n;i++){
			for(j=0;j<n;j++){
				s[i][j] = ' ';
			}
		}

		x = 0;
		y = n-1;
		d = 0;

		while(1){
			firstFlg = 1;
			cnt = 0;

			while(1){
				s[y][x] = '#';
				cnt++;

				nx = x + dx[d];
				ny = y + dy[d];
				if(nx < 0 || n <= nx || ny < 0 || n <= ny){
					d = (d + 1) % 4;
					break;
				}

				nnx = x + 2 * dx[d];
				nny = y + 2 * dy[d];
				if(0 <= nnx && nnx < n && 0 <= nny && nny < n && s[nny][nnx] == '#'){
					d = (d + 1) % 4;
					break;
				}

				x = nx;
				y = ny;
				firstFlg = 0;
			}
			if(firstFlg || cnt <= 2) break;
		}

		for(i=0;i<n;i++){
			for(j=0;j<n;j++){
				putchar(s[i][j]);
			}
			putchar('\n');
		}
	}

	return 0;
}