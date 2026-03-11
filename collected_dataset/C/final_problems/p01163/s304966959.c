#include <stdio.h>

int sosuu_num;
int sosuu_list[30000];
char is_sosuu[30001];

void make_sosuu_list(void) {
	int i,j;
	for(i=0;i<=30000;i++)is_sosuu[i]=1;
	is_sosuu[0]=is_sosuu[1]=0;
	sosuu_num=0;
	for(i=2;i<=30000;i++) {
		if(is_sosuu[i]) {
			sosuu_list[sosuu_num++]=i;
			for(j=i+i;j<=30000;j+=i)is_sosuu[j]=0;
		}
	}
}

int main(void) {
	int query;
	int i,j;
	int count;
	int a,b,c;
	make_sosuu_list();
	while(1) {
		scanf("%d",&query);
		if(query==0)break;
		count=0;
		for(i=0;i<sosuu_num;i++) {
			if(sosuu_list[i]>query)break;
			for(j=i;j<sosuu_num;j++) {
				a=sosuu_list[i];
				b=sosuu_list[j];
				c=query-a-b;
				if(c<b)break;
				if(is_sosuu[c]) {
					if(c>b-a && c<a+b)count++;
				}
			}
		}
		printf("%d\n",count);
	}
	return 0;
}