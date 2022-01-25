import { STATUSES, KINDS } from "../../../app/utils/books";

describe("Main page", () => {
  beforeEach(() => {
    // Re-seed database after each test.
    const port = Cypress.env("LLIBRES_PORT") || "8080";
    cy.exec(
      `ruby tests/fixtures/seed.rb tests/fixtures/test.yml http://localhost:${port}`,
      {
        env: { CI: true },
      }
    );
  });

  it("You can filter by read/unread/etc.", () => {
    cy.visit("/");

    cy.contains("td", "book1").should("not.exist");
    cy.contains("td", "book2").should("exist");

    cy.get("#status").select(`${STATUSES.read}`);

    cy.contains("td", "book1").should("exist");
    cy.contains("td", "book2").should("not.exist");
  });

  it("You can combine filters", () => {
    cy.visit("/");

    cy.contains("td", "book1").should("not.exist");
    cy.contains("td", "book3").should("not.exist");
    cy.contains("td", "book4").should("not.exist");

    cy.get("#status").select(`${STATUSES.read}`);

    cy.contains("td", "book1").should("exist");
    cy.contains("td", "book3").should("exist");
    cy.contains("td", "book4").should("exist");

    cy.get("#kind").select(`${KINDS.poetry}`);

    cy.contains("td", "book1").should("exist");
    cy.contains("td", "book3").should("not.exist");
    cy.contains("td", "book4").should("exist");

    cy.get("#filter").type("book1");

    cy.contains("td", "book1").should("exist");
    cy.contains("td", "book3").should("not.exist");
    cy.contains("td", "book4").should("not.exist");
  });

  it("You can edit a book", () => {
    cy.visit("/");

    // If you click a book, the modal dialog appears.
    cy.contains("td", "book2").click();
    cy.get("#edit").should("be.visible");
    cy.get("#update").should("not.be.visible");
    cy.get("#cancel").should("not.be.visible");
    cy.get("#delete").should("be.visible");

    // If you click 'edit', the set of buttons change.
    cy.get("#edit").click();
    cy.get("#edit").should("not.be.visible");
    cy.get("#update").should("be.visible");
    cy.get("#cancel").should("be.visible");
    cy.get("#delete").should("not.be.visible");

    // Let's show that you can go back.
    cy.get("#cancel").click();
    cy.get("#edit").should("be.visible");

    // Let's actually edit this book.
    cy.get("#edit").click();
    cy.get(".modal input").first().clear().type("new fancy name");
    cy.get("#update").click();

    // The results: dialog closes and we have a new title for the book.
    cy.contains("td", "book2").should("not.exist");
    cy.contains("td", "new fancy name").should("exist");
  });

  it("You can delete a book", () => {
    cy.visit("/");

    cy.contains("td", "book2").should("exist");

    cy.contains("td", "book2").click();
    cy.get("#delete").click();

    cy.contains("td", "book2").should("not.exist");
  });

  it("You can create a new book", () => {
    cy.visit("/");

    cy.get("#new-book").click();

    cy.get("#title").type("A new title");
    cy.get("#status").select(`${STATUSES.read}`);
    cy.get("#kind").select(`${KINDS.poetry}`);
    cy.get("#location").type("Casa").type("{enter}");
    cy.get("#author").type("New author").type("{enter}");
    cy.get("#date").click();
    cy.get("#create").click();

    cy.get("#create").should("not.exist");
    cy.get("#status").select(`${STATUSES.read}`);
    cy.contains("td", "A new title").should("exist");
  });
});
