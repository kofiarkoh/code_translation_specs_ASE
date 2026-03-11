#include <stdio.h>

int main(void)
{
	int i,j,k;
	int team[100],score[100];
	int rank[100];
	int count,number,tmp1,tmp2,info;
	
	count=0;
	//入力
	for(i=0;i<100;i++)
	{
		scanf("%d,%d",&team[i],&score[i]);
		if(team[i]+score[i]==0)
			break;
		count++;
	}
	
	//並び替え
	for(i=0;i<count;i++)
	{
		for(j=count-1;j>i;j--)
		{
			if(score[i]<=score[j])
			{
				tmp1=score[i];
				score[i]=score[j];
				score[j]=tmp1;
				
				tmp1=team[i];
				team[i]=team[j];
				team[j]=tmp1;
			}
		}
	}
	//順位付け
	number=1;
	rank[0]=number;
	tmp2=score[0];
	for(i=1;i<count;i++)
	{
		//printf("%d %d\n",score[i],tmp2);
		if(score[i]!=tmp2)
		{
			number++;
			tmp2=score[i];
		}
		rank[i]=number;
	}
	
	
	//問い合わせ
	while(scanf("%d",&info)!=EOF)
	{
		for(i=0;i<count;i++)
		{
			if(info==team[i])
				printf("%d\n",rank[i]);
			//printf("%d,%d\n",team[i],score[i]);
		}
	}
	return 0;
}