#include<stdio.h>

int count=0;
void input(int table[10][10]){
	int i,j;
	for(i=0;i<10;i++){
		for(j=0;j<10;j++){
			scanf("%d",&table[i][j]);
		}
	}
}

void output(int table[10][10]){
	int i,j;
	printf("output\n");
	for(i=0;i<10;i++){
		for(j=0;j<10;j++){
			printf("%d ",table[i][j]);
		}
		printf("\n");
	}
}

void separate(int num,int sep[]){
	sep[0]=num/1000;
	sep[1]=num/100%10;
	sep[2]=num/10%10;
	sep[3]=num%10;
}

void make_checkdigit(int table[10][10],int sep[5]){
	int a,b,c,d;
	a=table[0][sep[0]];
	b=table[a][sep[1]];
	c=table[b][sep[2]];
	d=table[c][sep[3]];
	sep[4]=d;
}

int check_checkdigit(int table[10][10],int sep[5]){
	int a,b,c,d,e;
	a=table[0][sep[0]];
	b=table[a][sep[1]];
	c=table[b][sep[2]];
	d=table[c][sep[3]];
	e=table[d][sep[4]];
	if(e==0) return 1;
	else return 0;
}

int check_humanerror_altering(int table[10][10],int sep[5],int err[5]){
	int i,j,c=1,d=1;
	for(i=0;i<5;i++){
		err[i]=sep[i];
	}
	for(i=0;i<5&c==1;i++){
		if(i>0) err[i-1]=sep[i-1];
		for(j=0;j<10&&c==1;j++){
			if(j==sep[i]){d=0;}
			if(d==1){
				err[i]=j;
				if(check_checkdigit(table,err)) c=0;
			}
			d=1;
		}
	}
	if(c==0){
		return 0;
	}
	if(c==1){
		return 1;
	}
}

void transporting(int *a,int *b){
	int tmp;
	tmp=*a;
	*a=*b;
	*b=tmp;
}

int check_humanerror_transporting(int table[10][10],int sep[5],int err[5]){
	int i,c=1,d=1;;
	for(i=0;i<5;i++){
		err[i]=sep[i];
	}
	for(i=0;i<4&&c==1;i++){
		if(err[i]==err[i+1]) d=0;
		if(d==1){
			transporting(&err[i],&err[i+1]);
			if(check_checkdigit(table,err)) c=0;
			if(c==1) transporting(&err[i],&err[i+1]);
		}
		d=1;
	}
	if(c==0){
		return 0;
	}
	if(c==1){
		return 1;
	}
}

int main(void){
	int table[10][10];
	int num,sep[5],err[5];
	int alt,tra;
	input(table);
	for(num=0;num<10000;num++){
		separate(num,sep);
		make_checkdigit(table,sep);
		alt=check_humanerror_altering(table,sep,err);
		tra=check_humanerror_transporting(table,sep,err);
		if(alt*tra==1);
		if(alt*tra==0) count++;
	}
	printf("%d\n",count);
	return 0;
}
