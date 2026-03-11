#include <stdio.h>

int matrix[15][15];

int visited[15][15];

void tansaku(int r,int c,int n) {
	if(r<0 || r>=n || c<0 || c>=n)return;
	if(visited[r][c])return;
	visited[r][c]=1;
	if(r>0 && matrix[r][c]==matrix[r-1][c])tansaku(r-1,c,n);
	if(r+1<n && matrix[r][c]==matrix[r+1][c])tansaku(r+1,c,n);
	if(c>0 && matrix[r][c]==matrix[r][c-1])tansaku(r,c-1,n);
	if(c+1<n && matrix[r][c]==matrix[r][c+1])tansaku(r,c+1,n);
}

int main(void) {
	int n,m;
	int i,j;
	int opCount;
	scanf("%d%d",&n,&m);
	for(i=0;i<n;i++) {
		for(j=0;j<n;j++)scanf("%d",&matrix[i][j]);
	}
	for(opCount=0;opCount<m;opCount++) {
		int query;
		int r,c,size,angle;
		scanf("%d",&query);
		switch(query) {
			case 0:
				scanf("%d%d%d%d",&r,&c,&size,&angle);r--;c--;
				for(;angle>0;angle-=90) {
					for(i=0;i<size;i++) {
						for(j=0;j<size;j++) {
							visited[i][j]=matrix[r+size-j-1][c+i];
						}
					}
					for(i=0;i<size;i++) {
						for(j=0;j<size;j++) {
							matrix[r+i][c+j]=visited[i][j];
						}
					}
				}
				break;
			case 1:
				scanf("%d%d%d",&r,&c,&size);r--;c--;
				for(i=0;i<size;i++) {
					for(j=0;j<size;j++) {
						matrix[r+i][c+j]=1-matrix[r+i][c+j];
					}
				}
				break;
			case 2:
				scanf("%d",&r);r--;
				c=matrix[r][0];
				for(i=0;i+1<n;i++)matrix[r][i]=matrix[r][i+1];
				matrix[r][n-1]=c;
				break;
			case 3:
				scanf("%d",&r);r--;
				c=matrix[r][n-1];
				for(i=n-1;i>0;i--)matrix[r][i]=matrix[r][i-1];
				matrix[r][0]=c;
				break;
			case 4:
				scanf("%d%d",&r,&c);r--;c--;
				for(i=0;i<n;i++) {
					for(j=0;j<n;j++)visited[i][j]=0;
				}
				tansaku(r,c,n);
				for(i=0;i<n;i++) {
					for(j=0;j<n;j++) {
						if(visited[i][j])matrix[i][j]=1-matrix[i][j];
					}
				}
				break;
		}
	}
	for(i=0;i<n;i++) {
		for(j=0;j<n;j++) {
			printf("%d%c",matrix[i][j],j+1<n?' ':'\n');
		}
	}
	return 0;
}