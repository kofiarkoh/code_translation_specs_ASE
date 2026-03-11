#include<stdio.h>
int ctoi(char *array){
	int num = 1,i=0,sum=0;
	while(array[i]!='\0'){
	switch(array[i]){
	case'2':
		num =2;
		break;
	case'3':
		num =3;
		break;
	case'4':
		num =4;
		break;
	case'5':
		num =5;
		break;
	case'6':
		num =6;
		break;
	case'7':
		num =7;
		break;
	case'8':
		num =8;
		break;
	case'9':
		num =9;
		break;
	case 'm':
		num *=1000;
		sum+=num;
		num=1;
		break;
	case 'c':
		num*=100;
		sum+=num;
		num=1;
		break;
	case 'x':
		num*=10;
		sum+=num;
		num=1;
		break;
	case 'i':
		sum+=num;
		num=1;
		break;	
	}
	i++;
	}
	return sum;
}

char* itoc(int sum){
	char array[200],*m="",*c="",*x="",*i="";
	int num;
	num = sum / 1000;
	switch(num){
	case 0:
		break;
	case 1:
		m="m";
		break;
	case 2:
		m = "2m";
		break;
	case 3:
		m = "3m";
		break;
	case 4:
		m = "4m";
		break;
	case 5:
		m = "5m";
		break;
	case 6:
		m = "6m";
		break;
	case 7:
		m = "7m";
		break;
	case 8:
		m = "8m";
		break;
	case 9:
		m = "9m";
		break;
	}
	num = sum / 100;
	num = num % 10;
	switch(num){
	case 0:
		break;
	case 1:
		c="c";
		  break;
	case 2:
		c = "2c";
		break;
	case 3:
		c = "3c";
		break;
	case 4:
		c = "4c";
		break;
	case 5:
		c = "5c";
		break;
	case 6:
		c = "6c";
		break;
	case 7:
		c = "7c";
		break;
	case 8:
		c = "8c";
		break;
	case 9:
		c = "9c";
		break;
	}
	num = sum % 100;
	num /= 10;
	switch(num){
	case 0:
		break;
	case 1:
		x="x";
		  break;
	case 2:
		x = "2x";
		break;
	case 3:
		x = "3x";
		break;
	case 4:
		x = "4x";
		break;
	case 5:
		x = "5x";
		break;
	case 6:
		x = "6x";
		break;
	case 7:
		x = "7x";
		break;
	case 8:
		x = "8x";
		break;
	case 9:
		x = "9x";
		break;
	}
	num = sum % 10;
	switch(num){
	case 0:
		break;
	case 1:
		i="i";
		break;
	case 2:
		i = "2i";
		break;
	case 3:
		i = "3i";
		break;
	case 4:
		i = "4i";
		break;
	case 5:
		i = "5i";
		break;
	case 6:
		i = "6i";
		break;
	case 7:
		i = "7i";
		break;
	case 8:
		i = "8i";
		break;
	case 9:
		i = "9i";
		break;
	}
	sprintf(array,"%s%s%s%s",m,c,x,i);
	return array;
}
int main(){
	int i,n,sum,lv,mv;
	char c[500],l[500],m[500];
	char *ans[501];
	scanf("%d",&n);
	for(i=0;i<n;i++){
		scanf("%s%s",l,m);
		ans[i] = itoc(ctoi(l)+ctoi(m));
		printf("%s",ans[i]);
		if(i<n-1)
		  putchar('\n');
	}
	return 0;
}