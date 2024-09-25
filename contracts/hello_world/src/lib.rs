#![allow(non_snake_case)]
#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Symbol, String, Env, log};

// Struct to represent an NFT Ticket
#[contracttype]
#[derive(Clone)]
pub struct NftTicket {
    pub ticket_id: u64,
    pub event_name: String,
    pub ticket_owner: String,
    pub price: u64,        // Price in Lumens (XLM)
    pub is_sold: bool,     // Indicates if the ticket has been sold
}

// Unique identifier for all tickets
const ALL_TICKETS: Symbol = symbol_short!("ALL_TIC");

// For generating unique ticket IDs
const COUNT_TICKET: Symbol = symbol_short!("C_TICKET");

#[contract]
pub struct NftTicketContract;

#[contractimpl]
impl NftTicketContract {

    // Function to create a new ticket for an event
    pub fn create_ticket(env: Env, event_name: String, price: u64) -> u64 {
        let mut count_ticket: u64 = env.storage().instance().get(&COUNT_TICKET).unwrap_or(0);
        count_ticket += 1;

        let ticket = NftTicket {
            ticket_id: count_ticket,
            event_name: event_name.clone(),
            ticket_owner: String::from_str(&env, "Platform"),  // Initially owned by the platform
            price,
            is_sold: false,  // Ticket is initially not sold
        };

        // Storing the ticket
        env.storage().instance().set(&ALL_TICKETS, &ticket);
        env.storage().instance().set(&COUNT_TICKET, &count_ticket);

        log!(&env, "Ticket Created: {} for Event: {}", count_ticket, event_name);
        count_ticket
    }

    // Function to buy a ticket
    pub fn buy_ticket(env: Env, ticket_id: u64, buyer: String) {
        let mut ticket = Self::view_ticket(env.clone(), ticket_id);

        if ticket.is_sold == true {
            panic!("Ticket has already been sold");
        }

        // Simulate payment with Lumens (XLM) (In a real implementation, handle payment processing)
        log!(&env, "Buyer {} is purchasing Ticket ID: {} for {} XLM", buyer.clone(), ticket_id, ticket.price);

        // Update ticket ownership and mark it as sold
        ticket.ticket_owner = buyer.clone();
        ticket.is_sold = true;

        // Update the ticket in storage
        env.storage().instance().set(&ALL_TICKETS, &ticket);

        log!(&env, "Ticket ID: {} sold to {}", ticket_id, buyer);
    }

    // Function to transfer a ticket to another user
    pub fn transfer_ticket(env: Env, ticket_id: u64, new_owner: String) {
        let mut ticket = Self::view_ticket(env.clone(), ticket_id);

        if ticket.is_sold == false {
            panic!("Ticket must be sold before transferring");
        }

        log!(&env, "Transferring Ticket ID: {} from {} to {}", ticket.ticket_owner.clone(), new_owner.clone());

        // Update ticket ownership
        ticket.ticket_owner = new_owner.clone();

        // Update the ticket in storage
        env.storage().instance().set(&ALL_TICKETS, &ticket);

        log!(&env, "Ticket ID: {} is now owned by {}", ticket_id, new_owner);
    }

    // View ticket details by ID
    pub fn view_ticket(env: Env, ticket_id: u64) -> NftTicket {
        env.storage().instance().get(&ALL_TICKETS).unwrap_or(NftTicket {
            ticket_id: 0,
            event_name: String::from_str(&env, "Not Found"),
            ticket_owner: String::from_str(&env, "Not Found"),
            price: 0,
            is_sold: false,
        })
    }
}
