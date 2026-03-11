#include<stdio.h>

void readwrite(int n,char s[][512],int i,int j){
	int ni,nj;
	char mem[512];
	for(nj = 0;nj < n;nj++){
		mem[nj] = s[i][nj];
	}
	for(ni = 0;ni < n;ni++){
		s[ni][j] = mem[ni];
	}
}

void copy(int n,const char f[][512],char t[][512]){
	int i,j;
	for(i = 0;i < n;i++){
		for(j = 0;j < n;j++){t[i][j] = f[i][j];}
	}
}

int check(int n,char s[][512]){
	int i,j;
	for(i = 0;i < n;i++){
		for(j = 0;j < n;j++){if(s[i][j] == '.'){return 0;}}
	}
	return 1;
}

void hand(int n,int c,int *r,char s[][512],int i,int j){
	c++;
	char m[512][512];
	int x,ni,nj;
	copy(n,s,m);
	readwrite(n,m,i,j);
	if(check(n,m) == 1){if(*r > c){*r = c;}}
	else if(2*n > c){
		for(ni = 0;ni < n;ni++){
			for(nj = 0;nj < n;nj++){hand(n,c,r,m,ni,nj);}
		}
	}
}

int main(void){
	int n,i,j,r = 114514;
	char s[512][512];
	scanf("%d",&n);
	for(i = 0;i < n;i++){scanf("%s",s[i]);}
	if(check(n,s) == 1){printf("0\n");return 0;}
	for(i = 0;i < n;i++){
		for(j = 0;j < n;j++){hand(n,0,&r,s,i,j);}
	}
	if(r == 114514){printf("-1\n");}else{printf("%d\n",r);}
	return 0;
}