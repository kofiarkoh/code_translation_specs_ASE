#include <stdio.h>
#include <stdlib.h>

typedef struct list_t_tag {
	int x,y,dx,dy;
	struct list_t_tag* next;
} list_t;

void add_tama(list_t* head,int x,int y) {
	static const int dxy[4][2]={{1,0},{-1,0},{0,1},{0,-1}};
	int i;
	list_t* head_next=head->next;
	list_t* now_head=head;
	for(i=0;i<4;i++) {
		now_head->next=malloc(sizeof(list_t));
		now_head->next->x=x;
		now_head->next->y=y;
		now_head->next->dx=dxy[i][0];
		now_head->next->dy=dxy[i][1];
		now_head=now_head->next;
	}
	now_head->next=head_next;
}

int main(void) {
	int board[4][4];
	int i,j;
	int nowans;
	for(i=0;i<4;i++) {
		for(j=0;j<4;j++)scanf("%d",&board[i][j]);
	}
	for(nowans=0;nowans<=5;nowans++) {
		int nowboard[4][4];
		int smax=(1<<(nowans*4));
		int search;
		int ok=0;
		for(search=0;search<smax;search++) {
			for(i=0;i<4;i++) {
				for(j=0;j<4;j++)nowboard[i][j]=board[i][j];
			}
			for(i=0;i<nowans;i++) {
				int x=(search>>(i*4+2))&3;
				int y=(search>>(i*4))&3;
				list_t head_buf={0,0,0,0,NULL};
				list_t* head=&head_buf;
				nowboard[y][x]++;
				if(nowboard[y][x]>=5) {
					nowboard[y][x]=0;
					add_tama(head,x,y);
				}
				while(head->next!=NULL) {
					list_t* now;
					list_t* prev;
					int xx,yy;
					/* move tama */
					for(now=head->next,prev=head;now!=NULL;) {
						now->x+=now->dx;
						now->y+=now->dy;
						if(now->x<0 || now->x>=4 || now->y<0 || now->y>=4) {
							list_t* now_next_bak=now->next;
							free(now);
							prev->next=now_next_bak;
							now=now_next_bak;
						} else if(nowboard[now->y][now->x]>0) {
							list_t* now_next_bak=now->next;
							nowboard[now->y][now->x]++;
							free(now);
							prev->next=now_next_bak;
							now=now_next_bak;
						} else {
							prev=now;
							now=now->next;
						}
					}
					/* erase drops */
					for(yy=0;yy<4;yy++) {
						for(xx=0;xx<4;xx++) {
							if(nowboard[yy][xx]>=5) {
								nowboard[yy][xx]=0;
								add_tama(head,xx,yy);
							}
						}
					}
				}
			}
			ok=1;
			for(i=0;i<4;i++) {
				for(j=0;j<4;j++) {
					if(nowboard[i][j]>0) {
						i=j=4;
						ok=0;
						break;
					}
				}
			}
			if(ok)break;
		}
		if(ok)break;
	}
	printf("%d\n",nowans<=5?nowans:-1);
	return 0;
}