#include <stdio.h>
#include <string.h>
#include <ctype.h>
#define MAX_MESSAGE_LEN 25
#define MAX_MESSENGERS_NUM 6

int n;
char message[MAX_MESSAGE_LEN + 1];

/* rotate message left by one */
void J()
{
    int i;
    char t = message[0];

    for(i = 0; i < n - 1; i++)
        message[i] = message[i + 1];
    message[i] = t;
}

/* rotate message right by one */
void C()
{
    int i;
    char t = message[n - 1];

    for(i = n - 1; i >= 0; i--)
        message[i] = message[i - 1];
    message[0] = t;
}

/* swap left and right halves of message */
void E()
{
    int half = n / 2;
    char buf[MAX_MESSAGE_LEN / 2];

    memcpy(buf, message, half);
    memcpy(message, message + half + (n % 2), half);
    memcpy(message + half + (n % 2), buf, half);
}

/* reverse message */
void A()
{
    int i, half = n / 2;
    char t;

    for(i = 0; i < half; i++)
    {
        t = message[i];
        message[i] = message[n - i - 1];
        message[n - i - 1] = t;
    }
}

/* increment digits in message */
void P()
{
    int i;

    for(i = 0; i < n; i++)
        if(isdigit((int)message[i]))
            message[i] = ((message[i] - '0') + 1) % 10 + '0';
}

/* decrement digits in message */
void M()
{
    int i;

    for(i = 0; i < n; i++)
        if(isdigit((int)message[i]))
            message[i] = ((message[i] - '0') + 9) % 10 + '0';
}

int main(void)
{
    int i;
    char messengers[MAX_MESSENGERS_NUM + 1];

    scanf("%*d%*c");

    while(~scanf("%s%*c", messengers))
    {
        scanf("%s%*c", message);
        n = strlen(message);

        for(i = strlen(messengers) - 1; i >= 0; i--)
            switch(messengers[i])
            {
              case 'J': C(); break;
              case 'C': J(); break;
              case 'E': E(); break;
              case 'A': A(); break;
              case 'P': M(); break;
              case 'M': P(); break;
            }

        puts(message);
    }

    return 0;
}