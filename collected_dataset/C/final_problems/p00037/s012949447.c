#include<stdio.h>
#define r(i,n) for(int i=0;i<n;++i)
int f[5][5];
int main()
{
	r(i,9){
		char s[6];
		scanf("%s",s);
		if(i%2){
			r(j,5){
				int a=s[j]-'0';
				f[i/2][j]|=a<<2;
				f[i/2+1][j]|=a;
			}
		}else{
			r(j,4){
				int a=s[j]-'0';
				f[i/2][j]|=a<<1;
				f[i/2][j+1]|=a<<3;
			}
		}
	}
	printf("R");
	int y=0,x=1,p=1;
	do{
		p=(p+2)%4;
		while(!((f[y][x]>>(p=(p+1)%4))&1));
		putchar("URDL"[p]);
		x+=p?2-p:0;
		y+=p==3?0:p-1;
	}while(y!=0||x!=0||f[0][0]==6&&p==3);
	puts("");
}