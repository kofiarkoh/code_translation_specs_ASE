#include<stdio.h>
#include<stdlib.h>
#include<math.h>
#include<string.h>

char map[71][71];
char answer[71],tmp[71];
int w,h;
int max;

void secretnum(int i,int j)
{
	int k;
	int len1,len2;
	int c;
	
	len1 = strlen(tmp);
	len2 = strlen(answer);
	tmp[len1] = map[i][j];
	tmp[len1+1] = '\0';
	
	c=-1;
	while(tmp[++c]=='0');
	for(k=c;k<=len1+1;k++){
		tmp[k-c]=tmp[k];
	}
	len1-=c;
	
	if(len1 + 1 > len2){
		strcpy(answer,tmp);
	}else if(len1 + 1 == len2){
		for(k=c;k<len1+1;k++){
			if(tmp[k] - answer[k] > 0){
				strcpy(answer,tmp);
				break;
			}else if(tmp[k] - answer[k] < 0){
				break;
			}
		}
	}
	if(i+1<h){
	  if(map[i+1][j] - '0' < 10){
			secretnum(i+1,j);
		}
	}
	if(j+1<w){
		if(map[i][j+1] - '0' < 10){
			secretnum(i,j+1);
		}
	}
	
	tmp[len1] = '\0';

	return;
}

int main(void)
{
	int i,j;
	int len;
	
	while(1)
	{
		scanf("%d %d ",&w,&h);
		if(w==0 && h==0)break;
		
		for(i=0;i<h;i++){
			scanf("%s ",map[i]);
		}

		answer[0]='\0';
		tmp[0]='\0';

		for(i=0;i<h;i++){
			for(j=0;j<w;j++){
				if(map[i][j] - '0' < 10){
					len = strlen(answer);
					if(i+j>len)secretnum(i,j);
				}
			}
		}
		
		printf("%s\n",answer);
	}
	return 0;
}