#include<stdio.h>
#include<string.h>
#include <ctype.h>
#include<stdlib.h>


int Integer(char *s) {

	int i,n,answer=0;
	char temp;

	for(i=0;i<strlen(s)/2;i++) {
		temp=s[i];
		s[i]=s[(strlen(s)-1)-i];
		s[(strlen(s)-1)-i]=temp;
	}

	i=0;
	while(i<=strlen(s)-1) {

		if(isdigit(s[i+1]))
			n=atoi(&s[i+1]);
		else
			n=1;


			switch(s[i]) {
			case 'i':
				answer+=n;
				break;
			case 'x':
				answer+=10*n;
				break;
			case 'c':
				answer+=100*n;
				break;
			case 'm':
				answer+=1000*n;
				break;
			}
			if(isdigit(s[i+1]))
				i+=2;
			else
				i++;
	}
	return answer;
}


int main() {

	int n;
	char s1[10],s2[10];
	int  sum,digit;
	char answer[8];
	int i,j;

	scanf("%d",&n);
	for(i=0;i<n;i++) {
		scanf("%s %s",s1,s2);
		sum=Integer(s1)+Integer(s2);

		for(j=0;j<8;j++)
			answer[j]='\0';

		digit=1000,j=0;
		while(sum>0) {
			if(sum/digit>0) {
				if(sum/digit>=2) {
					sprintf(&answer[j],"%d",sum/digit);
					j++;
				}
				if(digit==1000)
					answer[j]='m';
				else if(digit==100)
					answer[j]='c';
				else if(digit==10)
					answer[j]='x';
				else
					answer[j]='i';
				sum-=(sum/digit)*digit;
				j++;
			}
			digit/=10;
		}
		printf("%s\n",answer);
	}
	return 0;
}