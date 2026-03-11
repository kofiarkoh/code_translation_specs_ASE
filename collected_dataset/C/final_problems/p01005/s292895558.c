#include <stdio.h>

#define BIT_MAX 100000

int bit_table[BIT_MAX];

void bit_init() {
	int i;
	for(i=0;i<BIT_MAX;i++)bit_table[i]=0;
}

void bit_add(int pos,int value) {
	pos++;
	while(pos<=BIT_MAX) {
		bit_table[pos-1]+=value;
		pos+=pos & (-pos);
	}
}

int bit_sum(int pos) {
	int sum=0;
	pos++;
	while(pos>0) {
		sum+=bit_table[pos-1];
		pos-=pos & (-pos);
	}
	return sum;
}

long long invader_table[BIT_MAX];
long long invader_offset;
int invader_count;
int realtime_invader_count;

int getInvader(int id) {
	int left,right,mid;
	left=0;right=BIT_MAX-1;
	while(left<=right) {
		mid=(left+right)/2;
		if(bit_sum(mid)>=id+1)right=mid-1;
		else left=mid+1;
	}
	if(left>=BIT_MAX)return -1;
	return right+1;
}

int main(void) {
	int Q;
	long long L;
	int query,p1,p2;
	int query_count;
	int left,right,mid;
	int start_invader,next_invader;
	int count;
	while(scanf("%d%lld",&Q,&L)==2 && (Q|L)!=0) {
		bit_init();
		invader_offset=0;
		invader_count=0;
		realtime_invader_count=0;
		for(query_count=0;query_count<Q;query_count++) {
			scanf("%d",&query);
			switch(query) {
				case 0:
					invader_table[invader_count]=L-invader_offset;
					bit_add(invader_count,1);
					invader_count++;
					realtime_invader_count++;
					break;
				case 1:
					scanf("%d",&p1);
					invader_offset-=p1;
					count=0;
					while(1) {
						next_invader=getInvader(0);
						if(next_invader<0)break;
						if(invader_table[next_invader]+invader_offset<=0) {
							count++;
							bit_add(next_invader,-1);
							realtime_invader_count--;
						} else break;
					}
					if(count>0)printf("damage %d\n",count);
					break;
				case 2:
					scanf("%d",&p1);
					next_invader=getInvader(p1-1);
					if(next_invader>=0) {
						bit_add(next_invader,-1);
						realtime_invader_count--;
						puts("hit");
					} else puts("miss");
					break;
				case 3:
					scanf("%d%d",&p1,&p2);
					if(realtime_invader_count>0) {
						left=0;right=realtime_invader_count-1;
						while(left<=right) {
							mid=(left+right)/2;
							if(p1-p2<=invader_table[getInvader(mid)]+invader_offset) {
								right=mid-1;
							} else {
								left=mid+1;
							}
						}
						start_invader=right+1;
						left=0;right=realtime_invader_count-1;
						while(left<=right) {
							mid=(left+right)/2;
							if(p1+p2<invader_table[getInvader(mid)]+invader_offset) {
								right=mid-1;
							} else {
								left=mid+1;
							}
						}
						next_invader=right+1;
						for(count=start_invader;count<next_invader;count++) {
							bit_add(getInvader(start_invader),-1);
							realtime_invader_count--;
						}
						printf("bomb %d\n",next_invader-start_invader);
					} else puts("bomb 0");
					break;
				case 4:
					scanf("%d",&p1);
					next_invader=getInvader(p1-1);
					if(next_invader>=0) {
						printf("distance %lld\n",invader_table[next_invader]+invader_offset);
					} else puts("distance -1");
					break;
			}
		}
		puts("end");
	}
	return 0;
}